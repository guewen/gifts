use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

use rand::seq::SliceRandom;
use rand::thread_rng;

use serde::{Deserialize, Serialize};

// People in a group cannot exchange gifts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub people: Vec<Person>,
}

impl Group {
    pub fn new(people: Vec<Person>) -> Self {
        Self { people }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub email: String,
    pub name: String,
}

impl Person {
    pub fn new(email: &str, name: &str) -> Self {
        Self {
            email: email.to_string(),
            name: name.to_string(),
        }
    }
    // pub fn can_give_to(&self, receiver: &Person) -> bool {
    //     match &self.exclude {
    //         Some(exclude) => !exclude.iter().any(|x| *x == receiver.email),
    //         None => true,
    //     }
    // }
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.email == other.email && self.name == other.name
    }
}

impl Eq for Person {}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state);
        self.name.hash(state);
    }
}

#[derive(Debug, PartialEq)]
pub struct Pair {
    pub giver: Person,
    pub receiver: Person,
}

impl Pair {
    pub fn new(giver: Person, receiver: Person) -> Pair {
        Pair { giver, receiver }
    }
}

#[derive(Debug)]
pub struct Pool {
    groups: Vec<Group>,
}

impl Pool {
    pub fn new(groups: Vec<Group>) -> Self {
        Self { groups }
    }

    pub fn make_pairs(&mut self) -> Vec<Pair> {
        let mut rng = thread_rng();
        let mut pairs: Vec<Pair> = vec![];
        let mut restart_distribution = false;
        let mut tentatives = 0;
        let people = self
            .groups
            .iter()
            .map(|g| g.people.clone())
            .flatten()
            .collect::<Vec<Person>>();
        loop {
            // until we find a correct distribution
            tentatives += 1;
            if tentatives > 1000 {
                panic!("could not make pairs, probably due to recursive exclusions");
            }
            pairs.clear();
            let mut remaining = people.clone();
            let mut remaining_set: HashSet<Person> = remaining.iter().cloned().collect();
            remaining.shuffle(&mut rng);
            let mut receivers = people.iter().cycle();
            loop {
                if restart_distribution {
                    restart_distribution = false;
                    break;
                }
                let person = match remaining.pop() {
                    Some(person) => person,
                    None => break,
                };
                remaining_set.remove(&person);
                // TODO optimize with HashMap done once
                let our_group = self
                    .groups
                    .iter()
                    .find(|g| g.people.contains(&person))
                    .unwrap();
                let exclude = HashSet::from_iter(
                    our_group
                        .people
                        .clone()
                        .into_iter()
                        .filter(|p| p != &person),
                );
                if !remaining_set.is_empty() && remaining_set.is_subset(&exclude) {
                    // The only remaining candidates are excluded for this person!
                    // we start again the distribution
                    restart_distribution = true;
                    break;
                } else {
                    let receiver: &Person;
                    loop {
                        match receivers.next() {
                            Some(r) if r != &person => {
                                // TODO add method on Group
                                if !our_group.people.contains(&r)
                                {
                                    receiver = &r;
                                    break;
                                }
                            }
                            Some(_) => continue,
                            None => unreachable!(),
                        }
                    }
                    let pair = Pair::new(person.clone(), receiver.clone());
                    pairs.push(pair);
                }
            }
            if remaining.is_empty() {
                break;
            }
        }

        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn exclusion() {
    //     let a = Person::new("a@example.com", "A");
    //     let b = Person::new("b@example.com", "B");
    //     assert!(a.can_give_to(&b) == false);
    //     assert!(b.can_give_to(&a) == true);
    // }

    #[test]
    fn pool() {
        let p1 = Person::new("a@example.com", "A");
        let p2 = Person::new("b@example.com", "B");
        let p3 = Person::new("c@example.com", "C");
        let p4 = Person::new("d@example.com", "D");
        let group1 = Group::new(vec![p1.clone(), p2.clone()]);
        let group2 = Group::new(vec![p3.clone(), p4.clone()]);
        let mut pool = Pool::new(vec![group1, group2]);
        let pairs = pool.make_pairs();
        assert!(pairs.len() == 4, "pairs has a length of {}: {:#?}", pairs.len(), pairs);
        for pair in pairs.iter() {
            if pair.giver == p1 || pair.giver == p2 {
                assert!(pair.receiver == p3 || pair.receiver == p4);
            } else if pair.giver == p3 || pair.giver == p4 {
                assert!(pair.receiver == p1 || pair.receiver == p2);
            }
        }
    }

    #[test]
    fn person_equality() {
        let p1 = Person::new("a@example.com", "A");
        let p2 = Person::new("b@example.com", "B");
        assert!(p1 == p1);
        assert!(p2 == p2);
        assert!(p1 != p2);
    }

    #[test]
    fn pair_equality() {
        let p1 = Person::new("a@example.com", "A");
        let p2 = Person::new("b@example.com", "B");
        let p3 = Person::new("c@example.com", "C");
        let pair1 = Pair::new(p1.clone(), p2.clone());
        let pair2 = Pair::new(p1.clone(), p2.clone());
        let pair3 = Pair::new(p1.clone(), p3.clone());
        let pair4 = Pair::new(p2.clone(), p1.clone());
        assert!(pair1 == pair2);
        assert!(pair2 == pair1);
        assert!(pair1 != pair3);
        assert!(pair1 != pair4);
    }
}

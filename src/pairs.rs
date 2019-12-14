use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use rand::seq::SliceRandom;
use rand::thread_rng;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub email: String,
    pub name: String,
    pub exclude: Option<Vec<String>>, // list of emails
}

impl Person {
    pub fn new(email: &str, name: &str, exclude: Option<Vec<&str>>) -> Person {
        Person {
            email: email.to_string(),
            name: name.to_string(),
            exclude: exclude.map(|r| r.into_iter().map(|email| email.to_string()).collect()),
        }
    }
    pub fn can_give_to(&self, receiver: &Person) -> bool {
        match &self.exclude {
            Some(exclude) => !exclude.iter().any(|x| *x == receiver.email),
            None => true,
        }
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.email == other.email
    }
}

impl Eq for Person {}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state);
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
    people: Vec<Person>,
}

impl Pool {
    pub fn new(people: Vec<Person>) -> Pool {
        Pool { people }
    }

    pub fn make_pairs(&mut self) -> Vec<Pair> {
        let mut rng = thread_rng();
        let mut pairs: Vec<Pair> = vec![];
        let mut restart_distribution = false;
        let mut tentatives = 0;
        loop {
            // until we find a correct distribution
            tentatives += 1;
            if tentatives > 1000 {
                panic!("could not make pairs, probably due to recursive exclusions");
            }
            pairs.clear();
            let mut people = self.people.clone();
            let people_emails: HashSet<&str> =
                self.people.iter().map(|ref p| p.email.as_str()).collect();
            let mut consumed: Vec<&Person> = vec![];
            people.shuffle(&mut rng);
            let mut givers = people.iter();
            let mut receivers = people.iter().cycle();
            loop {
                if restart_distribution {
                    restart_distribution = false;
                    break;
                }
                let person = match givers.next() {
                    Some(person) => person,
                    None => break,
                };
                let consumed_emails: HashSet<&str> =
                    consumed.iter().map(|ref p| p.email.as_str()).collect();
                let mut remaining: HashSet<&str> = people_emails
                    .difference(&consumed_emails)
                    .cloned()
                    .collect();
                remaining.remove(&person.email.as_str());
                let exclude: HashSet<&str> = match &person.exclude {
                    Some(pexclude) => pexclude.iter().map(|s| s.as_str()).collect(),
                    None => HashSet::new(),
                };
                if remaining.is_subset(&exclude) {
                    // The only remaining candidates are excluded for this person!
                    // we start again the distribution
                    restart_distribution = true;
                    break;
                } else {
                    let receiver: &Person;
                    loop {
                        match receivers.next() {
                            Some(r) => {
                                if r != person && person.can_give_to(&r) && !consumed.contains(&r) {
                                    receiver = r;
                                    break;
                                }
                            }
                            None => unreachable!(),
                        }
                    }
                    consumed.push(receiver);
                    let pair = Pair::new(person.clone(), receiver.clone());
                    pairs.push(pair);
                }
            }
            if consumed.len() == people.len() {
                break;
            }
        }

        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exclusion() {
        let a = Person::new(
            "a@example.com",
            "A",
            Some(vec!["b@example.com"]),
        );
        let b = Person::new("b@example.com", "B", None);
        assert!(a.can_give_to(&b) == false);
        assert!(b.can_give_to(&a) == true);
    }

    // #[test]
    // fn pool() {
    //     let p1 = Person::new("a@example.com", "A", vec!["b@example.com"]);
    //     let p2 = Person::new("b@example.com", "B", vec![]);
    //     let p3 = Person::new("c@example.com", "C", vec![]);
    //     let p4 = Person::new("d@example.com", "D", vec!["b@example.com", "c@example.com"]);
    //     let mut pool = Pool::new(vec![&p1, &p2, &p3, &p4]);
    //     let pairs: Vec<Pair> = vec![];
    //     assert!(pool.make_pairs() == pairs);
    // }

    #[test]
    fn pair_equality() {
        let p1 = Person::new(
            "a@example.com",
            "A",
            Some(vec!["b@example.com"]),
        );
        let p2 = Person::new("b@example.com", "B", None);
        let p3 = Person::new("c@example.com", "C", None);
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

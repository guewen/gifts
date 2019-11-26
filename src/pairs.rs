use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use rand::{thread_rng, Rng};

#[derive(Debug,Clone)]
pub struct Person<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub exclude: Vec<&'a str>, // list of emails
}

impl<'a> Person<'a> {
    pub fn new(email: &'a str, name: &'a str, exclude: Vec<&'a str>) -> Person<'a> {
        Person {
            email: email,
            name: name,
            exclude: exclude,
        }
    }

    pub fn can_give_to(&self, receiver: &'a str) -> bool {
        !self.exclude.contains(&receiver)
    }
}

impl<'a> PartialEq for Person<'a> {
    fn eq(&self, other: &Person) -> bool {
        self.email == other.email
    }
}

impl<'a> Eq for Person<'a> { }


impl<'a> Hash for Person<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}


#[derive(Debug,PartialEq)]
pub struct Pair<'a> {
    pub giver: &'a Person<'a>,
    pub receiver: &'a Person<'a>,
}

impl<'a> Pair<'a> {
    pub fn new(giver: &'a Person<'a>, receiver: &'a Person<'a>) -> Pair<'a> {
        Pair {
            giver: giver,
            receiver: receiver,
        }
    }
}


#[derive(Debug)]
pub struct Pool<'a> {
    people: Vec<&'a Person<'a>>,
}

impl<'a> Pool<'a> {
    pub fn new(people: Vec<&'a Person<'a>>) -> Pool<'a> {
        Pool { people: people }
    }

    pub fn make_pairs(&mut self) -> Vec<Pair> {
        let mut rng = thread_rng();
        let mut pairs: Vec<Pair> = vec![];
        let mut restart_distribution = false;
        let mut tentatives = 0;
        loop {  // until we find a correct distribution
            tentatives += 1;
            if tentatives > 1000 {
                panic!("could not make pairs, probably due to recursive exclusions");
            }
            pairs.clear();
            let mut people = self.people.clone();
            let people_emails: HashSet<&str> = self.people.iter().map(|ref p| p.email).collect();
            let mut consumed: Vec<&Person> = vec![];
            rng.shuffle(&mut people);
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
                let consumed_emails: HashSet<&str> = consumed.iter().map(|ref p| p.email).collect();
                let mut remaining: HashSet<&str> = people_emails.difference(&consumed_emails).cloned().collect();
                remaining.remove(person.email);
                let exclude: HashSet<&str> = person.exclude.iter().cloned().collect();
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
                                if r != person && person.can_give_to(r.email) && !consumed.contains(&r) {
                                    receiver = r;
                                    break;
                                }
                            },
                            None => unreachable!()
                        }
                    }
                    consumed.push(receiver);
                    let pair = Pair::new(&person, &receiver);
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
        let a = Person::new("a@example.com", "A", vec!["b@example.com"]);
        let b = Person::new("b@example.com", "B", vec![]);
        assert!(a.can_give_to(b.email) == false);
        assert!(b.can_give_to(a.email) == true);
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
        let p1 = Person::new("a@example.com", "A", vec!["b@example.com"]);
        let p2 = Person::new("b@example.com", "B", vec![]);
        let p3 = Person::new("c@example.com", "C", vec![]);
        let pair1 = Pair::new(&p1, &p2);
        let pair2 = Pair::new(&p1, &p2);
        let pair3 = Pair::new(&p1, &p3);
        let pair4 = Pair::new(&p2, &p1);
        assert!(pair1 == pair2);
        assert!(pair2 == pair1);
        assert!(pair1 != pair3);
        assert!(pair1 != pair4);
    }
}

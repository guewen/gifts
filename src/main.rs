extern crate rand;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use rand::{thread_rng, Rng};

#[derive(Debug,Clone)]
pub struct Person<'a> {
    email: &'a str,
    name: &'a str,
    exclude: Vec<&'a str>, // list of emails
}

impl<'a> Person<'a> {
    fn new(email: &'a str, name: &'a str, exclude: Vec<&'a str>) -> Person<'a> {
        Person {
            email: email,
            name: name,
            exclude: exclude,
        }
    }

    fn can_give_to(&self, receiver: &'a str) -> bool {
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
    giver: &'a str,
    receiver: &'a str,
}

impl<'a> Pair<'a> {
    fn new(giver: &'a str, receiver: &'a str) -> Pair<'a> {
        Pair {
            giver: giver,
            receiver: receiver,
        }
    }
}


#[derive(Debug)]
pub struct Pool<'a> {
    people: Vec<Person<'a>>,
}

impl<'a> Pool<'a> {
    fn new(people: Vec<Person<'a>>) -> Pool<'a> {
        Pool { people: people }
    }

    fn make_pairs(&self) -> Vec<Pair> {
        let mut rng = thread_rng();
        let mut pairs: Vec<Pair> = vec![];
        let mut restart_distribution = false;
        loop {  // until we find a correct distribution
            pairs.clear();
            let mut people = self.people.clone();
            let people_emails: HashSet<&str> = people.iter().map(|ref p| p.email).collect();
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
                    let pair = Pair::new(person.email, receiver.email);
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

fn main() {
    let pool = Pool::new(vec![
        Person::new("a@example.com", "A", vec!["b@example.com"]),
        Person::new("b@example.com", "B", vec![]),
        Person::new("c@example.com", "C", vec![]),
        Person::new("d@example.com", "D", vec!["b@example.com", "c@example.com"]),
        Person::new("e@example.com", "E", vec![]),
        Person::new("f@example.com", "F", vec![]),
    ]);
    let pairs = pool.make_pairs();

    println!("{:?}", pairs);

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

    #[test]
    fn pool() {
        let pool = Pool::new(vec![
            Person::new("a@example.com", "A", vec!["b@example.com"]),
            Person::new("b@example.com", "B", vec![]),
            Person::new("c@example.com", "C", vec![]),
            Person::new("d@example.com", "D", vec!["b@example.com", "c@example.com"]),
        ]);
        let pairs: Vec<Pair> = vec![];
        assert!(pool.make_pairs() == pairs);
    }

    #[test]
    fn pair_equality() {
        let pair1 = Pair::new("a@example.com", "b@example.com");
        let pair2 = Pair::new("a@example.com", "b@example.com");
        let pair3 = Pair::new("a@example.com", "c@example.com");
        let pair4 = Pair::new("b@example.com", "a@example.com");
        assert!(pair1 == pair2);
        assert!(pair2 == pair1);
        assert!(pair1 != pair3);
        assert!(pair1 != pair4);
    }
}

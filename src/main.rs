#[derive(Debug)]
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


#[derive(Debug)]
struct Pair<'a> {
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

fn main() {
    let pairs = vec![Pair::new("abc@example.com", "def@example.com"),
                     Pair::new("def@example.com", "ghi@example.com")];

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
}

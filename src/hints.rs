use pairs;
use config;

#[derive(Debug)]
pub struct Hints {
    pub pairs: Vec<pairs::Pair>
}

impl Hints {
    pub fn new(pairs: Vec<pairs::Pair>) -> Self {
        Self { pairs }
    }

    pub fn secret_hints(&self, receiver: &pairs::Node) -> Vec<(config::Person, config::Person)> {
        let group_number = receiver.group_number;
        // find other people who give a gift to people of the same groups, so they can share
        // their gift if wanted
        let mut pairs = self.pairs.to_vec();
        pairs.retain(
            |pair| pair.receiver.group_number == group_number
                && pair.receiver.number != receiver.number
        );

        pairs.iter().map(|pair| (pair.giver.person.clone(), pair.receiver.person.clone())).collect()
    }
}

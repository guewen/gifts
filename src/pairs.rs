use rand::seq::SliceRandom;
use rand::thread_rng;

use std::collections::HashMap;

use config;

#[derive(Debug, Clone)]
pub struct Node {
    pub number: usize,
    pub group_number: usize,
    pub person: config::Person,
}

impl Node {
    pub fn new(number: usize, group_number: usize, person: config::Person) -> Self {
        Self { number, group_number, person }
    }
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub giver: Node,
    pub receiver: Node,
}

impl Pair {
    pub fn new(giver: Node, receiver: Node) -> Pair {
        Pair { giver, receiver }
    }
}

#[derive(Debug)]
pub struct Pool {
    indexed_nodes: HashMap<usize, Node>,
    max_attempts: u32,
}

impl Pool {
    pub fn new(nodes: Vec<Node>) -> Self {
        let mut indexed_nodes = HashMap::new();
        for node in nodes {
            indexed_nodes.insert(node.number, node);
        }
        Self {
            indexed_nodes: indexed_nodes,
            max_attempts: 1000
        }
    }

    pub fn make_pairs(&mut self) -> Result<Vec<Pair>, PoolError> {
        let mut rng = thread_rng();
        let mut pairs: Vec<(usize, usize)> = vec![];
        let node_numbers: Vec<usize> = self.indexed_nodes.keys().cloned().collect();

        // this should be an Halmitonian path based on a graph, but the naive brute-force
        // version will do for our small group
        'attempt: for _ in 0..self.max_attempts {
            pairs.clear();
            let mut givers_to_assign = node_numbers.clone();
            let mut receivers_to_assign = node_numbers.clone();
            givers_to_assign.shuffle(&mut rng);
            receivers_to_assign.shuffle(&mut rng);

            while let Some(giver_node_number) = givers_to_assign.pop() {
                if receivers_to_assign.is_empty() {
                    continue 'attempt
                }
                let receiver_node_number: usize = match receivers_to_assign.iter().find(
                    |&node_number| node_number != &giver_node_number &&
                        self.indexed_nodes[&giver_node_number].group_number != self.indexed_nodes[node_number].group_number
                ) {
                    Some(&node_number) => {
                        receivers_to_assign.retain(|&item| item != node_number);
                        node_number
                    },
                    None => continue 'attempt,  // failure to find a combination, maybe because of group constraints
                };

                pairs.push((giver_node_number, receiver_node_number));
            };
            if !receivers_to_assign.is_empty() {
                continue 'attempt
            }
            return Ok(self.node_pairs(pairs))
        }
        return Err(PoolError::AttemptsReached)
    }

    fn node_pairs(&self, pairs: Vec<(usize, usize)>) -> Vec<Pair> {
        pairs.iter().map(|t| Pair::new(self.indexed_nodes[&t.0].clone(), self.indexed_nodes[&t.1].clone())).collect()
    }
}

#[derive(Debug)]
pub enum PoolError {
    AttemptsReached,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool() {
        let p1 = config::Person::new("a@example.com", "A");
        let p2 = config::Person::new("b@example.com", "B");
        let p3 = config::Person::new("c@example.com", "C");
        let p4 = config::Person::new("d@example.com", "D");
        let group1 = config::Group::new(vec![p1.clone(), p2.clone()]);
        let group2 = config::Group::new(vec![p3.clone(), p4.clone()]);
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

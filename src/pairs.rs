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
        Self {
            number,
            group_number,
            person,
        }
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
            indexed_nodes,
            max_attempts: 1000,
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
                    continue 'attempt;
                }
                let receiver_node_number: usize =
                    match receivers_to_assign.iter().find(|&node_number| {
                        node_number != &giver_node_number
                            && self.indexed_nodes[&giver_node_number].group_number
                                != self.indexed_nodes[node_number].group_number
                    }) {
                        Some(&node_number) => {
                            receivers_to_assign.retain(|&item| item != node_number);
                            node_number
                        }
                        None => continue 'attempt, // failure to find a combination, maybe because of group constraints
                    };

                pairs.push((giver_node_number, receiver_node_number));
            }
            if !receivers_to_assign.is_empty() {
                continue 'attempt;
            }
            return Ok(self.node_pairs(pairs));
        }
        return Err(PoolError::AttemptsReached);
    }

    fn node_pairs(&self, pairs: Vec<(usize, usize)>) -> Vec<Pair> {
        pairs
            .iter()
            .map(|t| {
                Pair::new(
                    self.indexed_nodes[&t.0].clone(),
                    self.indexed_nodes[&t.1].clone(),
                )
            })
            .collect()
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
        let node1 = Node::new(0, 0, p1);
        let node2 = Node::new(1, 0, p2);
        let node3 = Node::new(2, 1, p3);
        let node4 = Node::new(3, 1, p4);
        // let group1 = config::Group::new(vec![p1.clone(), p2.clone()]);
        // let group2 = config::Group::new(vec![p3.clone(), p4.clone()]);
        let mut pool = Pool::new(vec![node1, node2, node3, node4]);
        let pairs = pool.make_pairs().unwrap();
        assert!(
            pairs.len() == 4,
            "pairs has a length of {}: {:#?}",
            pairs.len(),
            pairs
        );
        for pair in pairs.iter() {
            if pair.giver.number == 0 || pair.giver.number == 1 {
                assert!(pair.receiver.number == 2 || pair.receiver.number == 3);
            } else if pair.giver.number == 2 || pair.giver.number == 3 {
                assert!(pair.receiver.number == 0 || pair.receiver.number == 1);
            }
        }
    }
}

use std::fmt::Display;

use crate::primes::Primes;

pub struct PrimeTree<'a> {
    primes: &'a Primes,

    nodes: Vec<Node>,
}

struct Node {
    set: Option<NodeId>,
    link: Option<NodeId>,
    offset: Option<NodeId>,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.link.is_none() && self.set.is_none() && self.offset.is_none()
    }
}

#[derive(Clone, Copy)]
pub struct NodeId(usize);

impl<'a> PrimeTree<'a> {
    pub fn new(primes: &'a Primes) -> Self {
        Self {
            primes,
            nodes: Vec::new(),
        }
    }

    fn new_node(&mut self) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(Node {
            set: None,
            link: None,
            offset: None,
        });

        NodeId(next_index)
    }

    pub fn fill_with_num(&mut self, num: usize) {
        if num > 1 {
            self.inner_fill_with_num(num);
        }
    }

    fn inner_fill_with_num(&mut self, num: usize) -> NodeId {
        let node_id = self.new_node();

        let factors = self.primes.factorize(num).into_iter().enumerate();

        let len = factors.len();
        let mut id = node_id.0;
        let mut next_id = id;
        let mut num_offsets = 0;

        for (i, (prime_index, exponent)) in factors {
            self.nodes[id] = Node {
                set: if exponent > 1 {
                    Some(self.inner_fill_with_num(exponent))
                } else {
                    Some(self.new_node())
                },
                offset: if prime_index > num_offsets {
                    let ret = self.inner_fill_with_num(prime_index - num_offsets);
                    num_offsets += prime_index;
                    Some(ret)
                } else {
                    None
                },
                link: if i < (len - 1) {
                    num_offsets += 1;
                    next_id = self.new_node().0;
                    Some(NodeId(next_id))
                } else {
                    None
                },
            };
            id = next_id;
        }
        node_id
    }

    pub fn to_string(&self, NodeId(node): NodeId, indenet: usize) -> Option<String> {
        let current = &self.nodes.get(node)?;

        let set = if let Some(set) = current.set {
            self.to_string(set, indenet + 1)
        } else {
            None
        };
        let link = if let Some(link) = current.link {
            self.to_string(link, indenet + 1)
        } else {
            None
        };
        let offset = if let Some(offset) = current.offset {
            self.to_string(offset, indenet + 1)
        } else {
            None
        };

        if current.is_leaf() {
            return Some("*".to_string());
        }

        Some(format!(
            "\n{0}s{1}\n{0}l{2}\n{0}o{3}",
            "  ".repeat(indenet),
            set.unwrap_or("".to_string()),
            link.unwrap_or("".to_string()),
            offset.unwrap_or("".to_string())
        ))
    }
}

impl Display for PrimeTree<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.to_string(NodeId(0), 0).unwrap_or("".to_string())
        )
    }
}

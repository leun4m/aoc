use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct SimpleGraph<Node>
where
    Node: Eq + Hash + Clone,
{
    edges: HashMap<Node, Vec<Node>>,
}

impl<T> SimpleGraph<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        SimpleGraph {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        (*self.edges.entry(from).or_default()).push(to);
    }

    pub fn get_neighbour(&self, from: &T) -> Vec<T> {
        self.edges.get(from).unwrap().to_vec()
    }
}

impl<T, const N: usize> From<[(T, Vec<T>); N]> for SimpleGraph<T>
where
    T: Eq + Hash + Clone,
{
    fn from(arr: [(T, Vec<T>); N]) -> Self {
        SimpleGraph {
            edges: HashMap::from(arr),
        }
    }
}

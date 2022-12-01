use crate::graph::Graph;
use std::collections::HashMap;
use std::hash::Hash;

/// An unweighted graph with unidirectional edges.
#[derive(Debug, PartialEq, Eq)]
pub struct SimpleGraph<T>
where
    T: Eq + Hash + Clone,
{
    edges: HashMap<T, Vec<T>>,
}

impl<T> Graph<T> for SimpleGraph<T>
where
    T: Eq + Hash + Clone,
{
    fn new() -> Self {
        SimpleGraph {
            edges: HashMap::new(),
        }
    }

    fn neighbours(&self, node: &T) -> Vec<T> {
        self.edges.get(node).unwrap().clone()
    }

    fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    fn all_nodes(&self) -> Vec<T> {
        self.edges.keys().cloned().collect()
    }
}

impl<T> SimpleGraph<T>
where
    T: Eq + Hash + Clone,
{
    pub fn add_edge(&mut self, from: T, to: T) {
        (*self.edges.entry(from).or_default()).push(to);
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

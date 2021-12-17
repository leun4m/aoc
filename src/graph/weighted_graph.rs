use crate::graph::Graph;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct WeightedEdge<T, W>
where
    T: Eq + Hash,
    W: Eq + Hash,
{
    node: T,
    weight: W,
}

/// An unweighted graph with unidirectional edges.
#[derive(Debug, PartialEq, Eq)]
pub struct WeightedGraph<T, W>
where
    T: Eq + Hash + Clone,
    W: Eq + Hash,
{
    edges: HashMap<T, Vec<WeightedEdge<T, W>>>,
}

impl<T, W> Graph<T> for WeightedGraph<T, W>
where
    T: Eq + Hash + Clone,
    W: Eq + Hash,
{
    fn new() -> Self {
        WeightedGraph {
            edges: HashMap::new(),
        }
    }

    fn get_neighbours(&self, from: &T) -> Vec<T> {
        self.edges
            .get(from)
            .unwrap()
            .iter()
            .map(|x| x.node.clone())
            .collect()
    }

    fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    fn all_nodes(&self) -> Vec<T> {
        self.edges.keys().cloned().collect()
    }
}

impl<T, W> WeightedGraph<T, W>
where
    T: Eq + Hash + Clone,
    W: Eq + Hash + Clone,
{
    pub fn add_edge(&mut self, from: T, to: T, weight: W) {
        (*self.edges.entry(from).or_default()).push(WeightedEdge { node: to, weight });
    }

    pub fn get_weight(&self, from: &T, to: &T) -> Option<W> {
        if let Some(vec) = self.edges.get(from) {
            if let Some(edge) = vec.iter().find(|edge| edge.node == *to) {
                return Some(edge.weight.clone())
            }
        }

        None
    }
}

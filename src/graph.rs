//! A module containing generic [Graph](https://en.wikipedia.org/wiki/Graph_(discrete_mathematics))
//! implementations.

mod simple_graph;
mod weighted_graph;

pub use simple_graph::SimpleGraph;

pub use weighted_graph::WeightedGraph;

/// A Graph with nodes of Type `T`
pub trait Graph<T> {
    /// Creates a new, empty graph
    fn new() -> Self;

    /// Returns all neighbours from `node`
    fn neighbours(&self, node: &T) -> Vec<T>;

    /// Returns `true` if graph has no edges
    #[allow(dead_code)]
    fn is_empty(&self) -> bool;

    /// Returns all nodes
    fn all_nodes(&self) -> Vec<T>;
}

mod simple_graph;
mod weighted_graph;

pub use simple_graph::SimpleGraph;
pub use weighted_graph::WeightedEdge;
pub use weighted_graph::WeightedGraph;

/// A Graph
pub trait Graph<T> {
    /// Creates a new, empty graph
    fn new() -> Self;

    /// Returns all neighbours
    fn get_neighbours(&self, from: &T) -> Vec<T>;

    /// Returns true if graph has no edges
    fn is_empty(&self) -> bool;

    /// Returns all nodes
    fn all_nodes(&self) -> Vec<T>;
}

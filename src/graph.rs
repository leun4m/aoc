mod simple_graph;

pub use simple_graph::SimpleGraph;

/// A Graph
pub trait Graph<T> {
    /// Creates a new, empty graph
    fn new() -> Self;

    /// Returns all neighbours
    fn get_neighbours(&self, from: &T) -> Vec<T>;

    /// Returns true if graph has no edges 
    fn is_empty(&self) -> bool;
}

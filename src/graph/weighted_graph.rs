use crate::graph::Graph;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct WeightedEdge<T>
where
    T: Debug + Eq + Hash,
{
    node: T,
    weight: usize,
}

/// An unweighted graph with unidirectional edges.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WeightedGraph<T>
where
    T: Debug + Eq + Hash + Clone,
{
    edges: HashMap<T, Vec<WeightedEdge<T>>>,
}

impl<T> Graph<T> for WeightedGraph<T>
where
    T: Debug + Eq + Hash + Clone,
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

impl<T> WeightedGraph<T>
where
    T: Debug + Eq + Hash + Clone + Ord + Copy,
{
    pub fn add_edge(&mut self, from: T, to: T, weight: usize) {
        (*self.edges.entry(from).or_default()).push(WeightedEdge { node: to, weight });
    }

    pub fn get_edges(&self, from: &T) -> Vec<WeightedEdge<T>> {
        self.edges
            .get(from)
            .unwrap_or(&Vec::new())
            .to_vec()
    }

    /// Dijkstra's shortest path algorithm.
    ///
    /// Based on the [documentation for `std::collections::binary_heap`](https://doc.rust-lang.org/std/collections/binary_heap/index.html)
    pub fn shortest_path(&self, start: T, goal: T) -> Option<usize> {
        let mut dist: HashMap<T, usize> = self.edges.keys().map(|x| (*x, usize::MAX)).collect();
        let mut heap = BinaryHeap::new();

        *dist.entry(start).or_default() = 0;
        heap.push(State {
            cost: 0,
            node: start,
        });

        while let Some(State { cost, node }) = heap.pop() {
            if node == goal {
                return Some(cost);
            }

            if cost > *dist.get(&node).unwrap() {
                continue;
            }

            for edge in self.get_edges(&node) {
                let next = State {
                    cost: edge.weight + cost,
                    node: edge.node,
                };

                if next.cost < *dist.get(&next.node).unwrap() {
                    heap.push(next);
                    *dist.entry(next.node).or_default() = next.cost;
                }
            }
        }

        None
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<T, W> {
    node: T,
    cost: W,
}

impl<T, W> Ord for State<T, W>
where
    T: Ord,
    W: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<T, W> PartialOrd for State<T, W>
where
    T: Ord,
    W: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

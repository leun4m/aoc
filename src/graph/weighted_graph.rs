use crate::graph::Graph;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct WeightedEdge<T, W>
where
    T: Debug + Eq + Hash,
{
    node: T,
    weight: W,
}

/// A graph with weighted, unidirectional edges.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WeightedGraph<T, W>
where
    T: Debug + Eq + Hash + Clone,
{
    edges: HashMap<T, Vec<WeightedEdge<T, W>>>,
}

impl<T, W> Graph<T> for WeightedGraph<T, W>
where
    T: Debug + Eq + Hash + Clone,
{
    fn new() -> Self {
        WeightedGraph {
            edges: HashMap::new(),
        }
    }

    fn neighbours(&self, node: &T) -> Vec<T> {
        self.edges
            .get(node)
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
    T: Debug + Eq + Hash + Clone + Ord + Copy,
    W: Copy + Clone + Default + Ord + Add<Output = W>,
{
    pub fn add_edge(&mut self, from: T, to: T, weight: W) {
        (*self.edges.entry(from).or_default()).push(WeightedEdge { node: to, weight });
    }

    pub fn get_edges(&self, from: &T) -> Vec<WeightedEdge<T, W>> {
        self.edges.get(from).unwrap_or(&Vec::new()).clone()
    }

    /// Dijkstra's shortest path algorithm.
    ///
    /// Based on the [documentation for `std::collections::binary_heap`](https://doc.rust-lang.org/std/collections/binary_heap/index.html)
    pub fn shortest_path(&self, start: T, goal: T) -> Option<W> {
        let mut dist: HashMap<T, Distance<W>> = self
            .edges
            .keys()
            .map(|x| (*x, Distance::Infinite))
            .collect();
        let mut heap = BinaryHeap::new();

        *dist.entry(start).or_default() = Distance::Some(W::default());
        heap.push(State {
            cost: W::default(),
            node: start,
        });

        while let Some(State { cost, node }) = heap.pop() {
            if node == goal {
                return Some(cost);
            }

            if *dist.get(&node).unwrap() < cost {
                continue;
            }

            for edge in self.get_edges(&node) {
                let next = State {
                    cost: edge.weight + cost,
                    node: edge.node,
                };

                if *dist.get(&next.node).unwrap() > next.cost {
                    heap.push(next);
                    *dist.entry(next.node).or_default() = Distance::Some(next.cost);
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

#[derive(Default)]
enum Distance<W>
where
    W: Ord,
{
    Some(W),
    #[default]
    Infinite,
}

impl<W> PartialOrd<W> for Distance<W>
where
    W: Ord,
{
    fn partial_cmp(&self, other: &W) -> Option<Ordering> {
        match self {
            Distance::Some(w) => Some(w.cmp(other)),
            Distance::Infinite => Some(Ordering::Greater),
        }
    }
}

impl<W> PartialEq<W> for Distance<W>
where
    W: Ord,
{
    fn eq(&self, other: &W) -> bool {
        match self {
            Distance::Some(w) => w == other,
            Distance::Infinite => false,
        }
    }
}

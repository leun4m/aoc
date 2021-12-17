use crate::graph::{Graph, WeightedGraph};
use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) {
    let two_dim = parse(input);
    let graph_one = create_graph(&two_dim);
    println!("Part 1: {}", part_one(&graph_one));
    let two_dim_ext = extend(&two_dim);
    let graph_two = create_graph(&two_dim_ext);
    println!("Part 2: {}", part_one(&graph_two));
}

type Point = (usize, usize);
type RiskLevel = u64;
type Distances = HashMap<Point, RiskLevel>;
type Predecessors = HashMap<Point, Point>;
type Path = Vec<Point>;

fn parse(input: &str) -> Vec<Vec<RiskLevel>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

const REPEAT: usize = 5;

fn extend(two_dim: &[Vec<RiskLevel>]) -> Vec<Vec<RiskLevel>> {
    let mut result = Vec::new();

    for ry in 0..(two_dim.len() * REPEAT) {
        let fac_y = (ry / two_dim.len()) as u64;
        let y = ry % two_dim.len();

        let mut row = Vec::new();
        for rx in 0..(two_dim[y].len() * REPEAT) {
            let fac_x = (rx / two_dim[y].len()) as u64;
            let x = rx % two_dim[y].len();

            let mut risk_level = two_dim[y][x] + (fac_x + fac_y);
            if risk_level > 9 {
                risk_level = risk_level % 10 + 1;
            }

            row.push(risk_level);
        }
        result.push(row);
    }

    result
}

fn create_graph(two_dim: &[Vec<RiskLevel>]) -> WeightedGraph<Point, RiskLevel> {
    let mut graph = WeightedGraph::new();

    for y in 0..two_dim.len() {
        for x in 0..two_dim[y].len() {
            let risk_level = two_dim[y][x];
            if 0 < x {
                graph.add_edge((x - 1, y), (x, y), risk_level);
            }
            if 0 < y {
                graph.add_edge((x, y - 1), (x, y), risk_level);
            }
            if x + 1 < two_dim.len() {
                graph.add_edge((x + 1, y), (x, y), risk_level);
            }
            if y + 1 < two_dim[x].len() {
                graph.add_edge((x, y + 1), (x, y), risk_level);
            }
        }
    }

    graph
}

fn part_one(graph: &WeightedGraph<Point, RiskLevel>) -> u64 {
    log::trace!("part_one entered");
    let aim = get_bottom_right(graph);
    log::trace!("found aim: {:?}", aim);
    let predecessors = dijkstra(graph, (0, 0));
    log::trace!("dijkstra finished");
    let path = find_shortest_path(aim, &predecessors);
    log::trace!("find_shortes_path finished");

    path.windows(2)
        .map(|x| graph.get_weight(&x[0], &x[1]).unwrap())
        .sum()
}

fn get_bottom_right(graph: &WeightedGraph<Point, RiskLevel>) -> Point {
    let max_x = graph
        .all_nodes()
        .iter()
        .map(|a| a.0)
        .max()
        .unwrap();
    let max_y = graph
        .all_nodes()
        .iter()
        .map(|a| a.1)
        .max()
        .unwrap();
    (max_x, max_y)
}

fn dijkstra(graph: &WeightedGraph<Point, RiskLevel>, start: Point) -> Predecessors {
    log::trace!("Start collecting nodes");
    let mut nodes: HashSet<Point> = HashSet::from_iter(graph.all_nodes());
    log::trace!("Create distance");
    let mut distances = Distances::new();
    log::trace!("Create predecessors");
    let mut predecessors: Predecessors = Predecessors::new();

    log::trace!("Iterate nodes");
    for node in nodes.iter() {
        distances.insert(*node, RiskLevel::MAX);
    }

    distances.insert(start, 0);

    let start_size = nodes.len();

    log::trace!("Start analyzing nodes: {}", start_size);

    while !nodes.is_empty() {
        let node = *distances
            .iter()
            .filter(|(p, _)| nodes.contains(p))
            .min_by_key(|(_, d)| *d)
            .unwrap()
            .0;
        nodes.remove(&node);

        for neighbor in graph.get_neighbours(&node)
        {
            if nodes.contains(&neighbor) {
                distance_update(node, neighbor, graph, &mut distances, &mut predecessors);
            }
        }

        if nodes.len() % cmp::max(start_size / 1000, 1) == 0 {
            log::trace!(
                "Nodes analyzed: {:.2} %",
                (start_size - nodes.len()) as f64 / start_size as f64 * 100.0
            )
        }
    }

    predecessors
}

fn distance_update(
    u: Point,
    v: Point,
    graph: &WeightedGraph<Point, RiskLevel>,
    distances: &mut Distances,
    predecessors: &mut Predecessors,
) {
    let alternative = distances.get(&u).unwrap() + graph.get_weight(&u, &v).unwrap();
    if alternative < *distances.get(&v).unwrap() {
        distances.insert(v, alternative);
        predecessors.insert(v, u);
    }
}

fn find_shortest_path(aim: Point, predecessors: &Predecessors) -> Path {
    let mut path = vec![aim];
    let mut u = aim;
    while let Some(v) = predecessors.get(&u) {
        u = *v;
        path.push(u);
    }
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
    1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";

    #[test]
    fn extend_works() {
        let input = vec![vec![8]];
        let output = vec![
            vec![8, 9, 1, 2, 3],
            vec![9, 1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6],
            vec![3, 4, 5, 6, 7],
        ];
        assert_eq!(extend(&input), output);
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&create_graph(&parse(INPUT))), 40);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_one(&create_graph(&extend(&parse(INPUT)))), 315);
    }
}

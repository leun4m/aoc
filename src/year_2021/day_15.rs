use std::collections::HashMap;
use std::collections::HashSet;
use log;

pub fn solve(input: &str) {
    let two_dim = parse(input);
    let graph_one = create_graph(&two_dim);
    println!("Part 1: {}", part_one(&graph_one));
}

type Point = (usize, usize);
type Edge = (Point, Point);
type RiskLevel = u64;
type Graph = HashMap<Edge, RiskLevel>;
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

fn create_graph(two_dim: &[Vec<RiskLevel>]) -> Graph { 
    let mut graph = Graph::new();

    for x in 0..two_dim.len() {
        for y in 0..two_dim[x].len() {
            let risk_level = two_dim[x][y];
            if 0 < x {
                graph.insert(((x - 1, y), (x, y)), risk_level);
            }
            if 0 < y {
                graph.insert(((x, y - 1), (x, y)), risk_level);
            }
            if x + 1 < two_dim.len() {
                graph.insert(((x + 1, y), (x, y)), risk_level);
            }
            if y + 1 < two_dim[x].len() {
                graph.insert(((x, y + 1), (x, y)), risk_level);
            }
        }
    }

    graph
}

fn part_one(graph: &Graph) -> u64 {
    log::trace!("part_one entered");
    let aim = get_bottom_right(graph);
    log::trace!("found aim: {:?}", aim);
    let predecessors = dijkstra(graph, (0, 0));
    log::trace!("dijkstra finished");
    let path = find_shortest_path(aim, &predecessors);
    log::trace!("find_shortes_path finished");

    path.windows(2)
        .map(|x| graph.get(&(x[0], x[1])).unwrap())
        .sum()
}

fn get_bottom_right(graph: &Graph) -> Point {
    let max_x = graph.keys().map(|(from, _)| *from).map(|a| a.0).max().unwrap();
    let max_y = graph.keys().map(|(from, _)| *from).map(|a| a.1).max().unwrap();
    (max_x, max_y)
}

fn dijkstra(graph: &Graph, start: Point) -> Predecessors {
    log::trace!("Start collecting nodes"); 
    let mut nodes: HashSet<Point> = graph.keys().map(|(from, _)| *from).collect();
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

        for neighbor in graph
            .iter()
            .map(|(edge, _)| edge)
            .filter(|(from, _)| *from == node)
            .map(|(_, to)| to)
        {
            if nodes.contains(neighbor) {
                distance_update(node, *neighbor, graph, &mut distances, &mut predecessors);
            }
        }

        if nodes.len() % (start_size / 20) == 0 {
            log::trace!("Nodes analyzed: {:.2} %", (start_size - nodes.len()) as f64 / start_size as f64 * 100.0)
        }
    }

    predecessors
}

fn distance_update(
    u: Point,
    v: Point,
    graph: &Graph,
    distances: &mut Distances,
    predecessors: &mut Predecessors,
) {
    let alternative = distances.get(&u).unwrap() + graph.get(&(u, v)).unwrap();
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
    fn part_one_works() {
        assert_eq!(part_one(&create_graph(&parse(INPUT))), 40);
    }
}
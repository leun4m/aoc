use crate::graph::{Graph, WeightedGraph};

pub fn solve(input: &str) {
    let matrix = parse(input);
    println!("Part 1: {}", part_one(&matrix));
    println!("Part 2: {}", part_two(&matrix));
}

type Point = (usize, usize);
type RiskLevel = usize;

const EXTEND_REPEAT: usize = 5;

fn parse(input: &str) -> Vec<Vec<RiskLevel>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::trim)
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn part_one(matrix: &[Vec<RiskLevel>]) -> RiskLevel {
    let graph = create_graph(matrix);
    let aim = get_bottom_right(&graph);
    graph.shortest_path((0, 0), aim).unwrap()
}

fn part_two(matrix: &[Vec<RiskLevel>]) -> RiskLevel {
    let matrix_extended = extend(matrix);
    let graph = create_graph(&matrix_extended);
    let aim = get_bottom_right(&graph);
    graph.shortest_path((0, 0), aim).unwrap()
}

fn get_bottom_right(graph: &WeightedGraph<Point, RiskLevel>) -> Point {
    let max_x = graph.all_nodes().iter().map(|a| a.0).max().unwrap();
    let max_y = graph.all_nodes().iter().map(|a| a.1).max().unwrap();
    (max_x, max_y)
}

fn extend(two_dim: &[Vec<RiskLevel>]) -> Vec<Vec<RiskLevel>> {
    let mut result = Vec::new();

    for ry in 0..(two_dim.len() * EXTEND_REPEAT) {
        let fac_y = (ry / two_dim.len()) as RiskLevel;
        let y = ry % two_dim.len();

        let mut row = Vec::new();
        for rx in 0..(two_dim[y].len() * EXTEND_REPEAT) {
            let fac_x = (rx / two_dim[y].len()) as RiskLevel;
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
        assert_eq!(part_one(&parse(INPUT)), 40);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(&parse(INPUT)), 315);
    }
}

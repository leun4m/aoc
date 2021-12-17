use crate::graph::SimpleGraph;
use itertools::Itertools;

pub fn solve(input: &str) {
    let graph = parse(input);
    println!("Part 1: {}", part_one(&graph));
    println!("Part 2: {}", part_two(&graph));
}

type Cave<'a> = &'a str;

const START: &str = "start";
const END: &str = "end";

fn parse(input: &str) -> SimpleGraph<Cave> {
    let mut graph = SimpleGraph::new();

    for (a, b) in input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_pair(line))
    {
        graph.add_edge(a, b);
        graph.add_edge(b, a);
    }

    graph
}

fn parse_pair(input: &str) -> (Cave, Cave) {
    let words: Vec<&str> = input.trim().split('-').collect();
    (words[0], words[1])
}

fn part_one(graph: &SimpleGraph<Cave>) -> usize {
    endings(graph, &[], START, can_been_visited_again_one).len()
}

fn part_two(graph: &SimpleGraph<Cave>) -> usize {
    endings(graph, &[], START, can_been_visited_again_two).len()
}

fn can_been_visited_again_one(cave: Cave, visited: &[Cave]) -> bool {
    is_big_cave(cave) || !visited.contains(&cave)
}

fn can_been_visited_again_two(cave: Cave, visited: &[Cave]) -> bool {
    if is_big_cave(cave) || !visited.contains(&cave) {
        return true;
    }

    let small_caves = visited.iter().filter(|x| !is_big_cave(x)).count();
    let unique_small_caves = visited.iter().filter(|x| !is_big_cave(x)).unique().count();

    cave != START && cave != END && unique_small_caves == small_caves
}

fn endings<'a, F>(
    graph: &'a SimpleGraph<Cave>,
    visited: &[Cave],
    start: Cave,
    can_be_visited: F,
) -> Vec<Vec<Cave<'a>>>
where
    F: Copy + Fn(&str, &[Cave]) -> bool,
{
    if start == END {
        vec![vec![END]]
    } else {
        let mut new_visited = visited.to_owned();
        new_visited.push(start);

        graph
            .get_neighbour(&start)
            .iter()
            .filter(|next| can_be_visited(next, &new_visited))
            .flat_map(|next| endings(graph, &new_visited, next, can_be_visited))
            .collect()
    }
}

fn is_big_cave(cave: Cave) -> bool {
    cave == cave.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";

    #[test]
    fn parse_works() {
        assert_eq!(
            parse(
                "start-A
            start-b"
            ),
            SimpleGraph::from([
                ("start", vec!["A", "b"]),
                ("A", vec!["start"]),
                ("b", vec!["start"])
            ])
        );
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&parse(INPUT)), 10);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(&parse(INPUT)), 36)
    }
}

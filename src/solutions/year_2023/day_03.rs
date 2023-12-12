use itertools::Itertools;

pub fn solve(input: &str) {
    let map = parse(input);
    println!("Part 1: {}", part_one(&map));
}

fn part_one(schematic: &Schematic) -> usize {
    let vec = schematic
        .map
        .iter()
        .filter(|x| !x.1.is_symbol())
        .filter(|x| x.0.iter().any(|z| is_adjacent(z, schematic)))
        .map(|x| x.1.get_value())
        .collect_vec();

    println!("{:?}", vec);

    vec.iter().sum()
}

fn is_adjacent(point: &Point, schematic: &Schematic) -> bool {
    get_symbols(&schematic.map)
        .iter()
        .flat_map(|thing| thing.0.iter())
        .any(|x| next_to(x, point))
}

fn next_to(a: &Point, b: &Point) -> bool {
    [
        Point(a.0 + 1, a.1 + 1),
        Point(a.0 + 1, a.1 - 1),
        Point(a.0 - 1, a.1 - 1),
        Point(a.0 - 1, a.1 + 1),
        Point(a.0 + 1, a.1),
        Point(a.0 - 1, a.1),
        Point(a.0, a.1 + 1),
        Point(a.0, a.1 - 1),
    ]
    .iter()
    .any(|x| x == b)
}

fn get_symbols(map: &[Thing]) -> Vec<&Thing> {
    map.iter().filter(|x| x.1.is_symbol()).collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Point(usize, usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Element {
    Number(usize),
    Symbol(char),
}

impl Element {
    fn is_symbol(&self) -> bool {
        matches!(self, Element::Symbol(_))
    }

    fn get_value(&self) -> usize {
        match self {
            Element::Number(z) => *z,
            Element::Symbol(_) => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Schematic {
    map: Vec<Thing>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Thing(Vec<Point>, Element);

impl Schematic {
    fn new(map: Vec<Thing>) -> Self {
        let width = map
            .iter()
            .flat_map(|x| x.0.iter())
            .map(|x| x.0)
            .max()
            .unwrap_or_default();
        let height = map
            .iter()
            .flat_map(|x| x.0.iter())
            .map(|x| x.1)
            .max()
            .unwrap_or_default();

        Self { map, width, height }
    }
}

fn parse(input: &str) -> Schematic {
    let mut map = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut number = String::new();
        for (x, symbol) in line.chars().enumerate() {
            if symbol.is_ascii_digit() {
                number.push(symbol);
            } else {
                if !number.is_empty() {
                    let points = (x - number.len()..x).map(|x0| Point(x0, y)).collect_vec();

                    map.push(Thing(
                        points,
                        Element::Number(number.parse::<usize>().unwrap()),
                    ));
                    number.clear();
                }

                if symbol != '.' {
                    map.push(Thing(vec![Point(x, y)], Element::Symbol(symbol)));
                }
            }
        }
    }

    Schematic::new(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_parse() {
        let map = Schematic::new(Vec::from([
            Thing(
                vec![Point(0, 0), Point(1, 0), Point(2, 0)],
                Element::Number(467),
            ),
            Thing(
                vec![Point(5, 0), Point(6, 0), Point(7, 0)],
                Element::Number(114),
            ),
            Thing(vec![Point(3, 1)], Element::Symbol('*')),
            Thing(vec![Point(2, 2), Point(3, 2)], Element::Number(35)),
            Thing(
                vec![Point(6, 2), Point(7, 2), Point(8, 2)],
                Element::Number(633),
            ),
            Thing(vec![Point(6, 3)], Element::Symbol('#')),
            Thing(
                vec![Point(0, 4), Point(1, 4), Point(2, 4)],
                Element::Number(617),
            ),
            Thing(vec![Point(3, 4)], Element::Symbol('*')),
            Thing(vec![Point(5, 5)], Element::Symbol('+')),
            Thing(vec![Point(7, 5), Point(8, 5)], Element::Number(58)),
            Thing(
                vec![Point(2, 6), Point(3, 6), Point(4, 6)],
                Element::Number(592),
            ),
            Thing(
                vec![Point(6, 7), Point(7, 7), Point(8, 7)],
                Element::Number(755),
            ),
            Thing(vec![Point(3, 8)], Element::Symbol('$')),
            Thing(vec![Point(5, 8)], Element::Symbol('*')),
            Thing(
                vec![Point(1, 9), Point(2, 9), Point(3, 9)],
                Element::Number(664),
            ),
            Thing(
                vec![Point(5, 9), Point(6, 9), Point(7, 9)],
                Element::Number(598),
            ),
        ]));
        assert_eq!(map, parse(EXAMPLE_INPUT))
    }

    #[test]
    fn test_part_one() {
        assert_eq!(4361, part_one(&parse(EXAMPLE_INPUT)));
    }
}

use std::collections::HashMap;

pub fn solve(input: &str) {
    let lines = parse(input);
    println!("Part 1: {}", part_one(&lines));
    println!("Part 2: {}", part_two(&lines));
}

type Base = i32;
type Point = (Base, Base);
type Line = (Point, Point);

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(input: &str) -> Line {
    let points: Vec<&str> = input.trim().split("->").collect();
    (parse_point(points[0]), parse_point(points[1]))
}

fn parse_point(point: &str) -> Point {
    let point: Vec<&str> = point.trim().split(',').collect();
    (point[0].parse().unwrap(), point[1].parse().unwrap())
}

fn part_one(lines: &[Line]) -> usize {
    let filtered_lines: Vec<&Line> = lines
        .iter()
        .filter(|x| is_horizontal_or_vertical(x))
        .collect();
    calc_crosspoints(&filtered_lines)
}

fn part_two(lines: &[Line]) -> usize {
    let unfiltered_lines: Vec<&Line> = lines.iter().collect();
    calc_crosspoints(&unfiltered_lines)
}

fn calc_crosspoints(lines: &[&Line]) -> usize {
    let points: Vec<Point> = lines.iter().flat_map(|line| get_points(line)).collect();
    let mut point_counts: HashMap<Point, u32> = HashMap::new();

    for point in points {
        *point_counts.entry(point).or_insert(0) += 1;
    }

    point_counts.values().filter(|&&x| x > 1).count()
}

fn is_horizontal_or_vertical(line: &Line) -> bool {
    line.0 .0 == line.1 .0 || line.0 .1 == line.1 .1
}

fn get_points(line: &Line) -> Vec<Point> {
    let mut result = Vec::new();
    let point_a = line.0;
    let point_b = line.1;

    if is_horizontal_or_vertical(line) {
        let (x1, x2) = if point_a.0 <= point_b.0 {
            (point_a.0, point_b.0)
        } else {
            (point_b.0, point_a.0)
        };
        let (y1, y2) = if point_a.1 <= point_b.1 {
            (point_a.1, point_b.1)
        } else {
            (point_b.1, point_a.1)
        };

        for x in x1..=x2 {
            for y in y1..=y2 {
                result.push((x, y))
            }
        }
    } else {
        let mut xc = point_a.0;
        let mut yc = point_a.1;

        let xmod = if point_a.0 < point_b.0 { 1 } else { -1 };
        let ymod = if point_a.1 < point_b.1 { 1 } else { -1 };

        while xc != point_b.0 && yc != point_b.1 {
            result.push((xc, yc));

            xc += xmod;
            yc += ymod;
        }

        result.push((point_b.0, point_b.1))
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    #[test]
    fn parse_works() {
        let output = parse(INPUT);
        assert_eq!(
            output,
            vec![
                ((0, 9), (5, 9)),
                ((8, 0), (0, 8)),
                ((9, 4), (3, 4)),
                ((2, 2), (2, 1)),
                ((7, 0), (7, 4)),
                ((6, 4), (2, 0)),
                ((0, 9), (2, 9)),
                ((3, 4), (1, 4)),
                ((0, 0), (8, 8)),
                ((5, 5), (8, 2))
            ]
        )
    }

    #[test]
    fn get_points_works() {
        assert_eq!(get_points(&((1, 1), (3, 3))), vec![(1, 1), (2, 2), (3, 3)]);
    }

    #[test]
    fn part_one_works() {
        let parsed = parse(INPUT);
        assert_eq!(part_one(&parsed), 5);
    }
    #[test]
    fn part_two_works() {
        let parsed = parse(INPUT);
        assert_eq!(part_two(&parsed), 12);
    }
}

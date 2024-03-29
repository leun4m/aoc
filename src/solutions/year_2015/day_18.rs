use crate::parser;

pub fn solve(input: &str) {
    let lights = parse(input);
    println!("Part 1: {}", part_one(&lights, 100));
    println!("Part 2: {}", part_two(&lights, 100));
}

type LightRow = Vec<bool>;
type LightGrid = Vec<LightRow>;

fn parse(input: &str) -> LightGrid {
    parser::lines_custom(input, parse_line)
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("Unexpected char: {c}"),
        })
        .collect::<Vec<_>>()
}

fn part_one(start: &[LightRow], steps: u32) -> usize {
    let switch_on = |x: usize, y: usize, lights: &[LightRow]| {
        let neighbours = count_neighbours(lights, x as isize, y as isize);
        neighbours == 3 || (neighbours == 2 && lights[x][y])
    };

    iterate(start, steps, switch_on)
}

fn part_two(start: &[LightRow], steps: u32) -> usize {
    let switch_on = |x: usize, y: usize, lights: &[LightRow]| {
        let neighbours = count_neighbours(lights, x as isize, y as isize);
        neighbours == 3 || (neighbours == 2 && lights[x][y]) || is_corner(x, y, lights)
    };

    iterate(start, steps, switch_on)
}

fn iterate<F>(start: &[LightRow], steps: u32, switch_on: F) -> usize
where
    F: Copy + Fn(usize, usize, &[LightRow]) -> bool,
{
    (0..steps)
        .collect::<Vec<_>>()
        .iter()
        .fold(start.to_owned(), |acc, _| next_step(&acc, switch_on))
        .into_iter()
        .flatten()
        .filter(|&x| x)
        .count()
}

fn next_step<F>(lights: &[LightRow], switch_on: F) -> LightGrid
where
    F: Copy + Fn(usize, usize, &[LightRow]) -> bool,
{
    lights
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, _)| switch_on(x, y, lights))
                .collect()
        })
        .collect()
}

fn count_neighbours(lights: &[LightRow], x: isize, y: isize) -> u32 {
    let mut result = 0;
    result += numeric(lights, x - 1, y - 1);
    result += numeric(lights, x - 1, y);
    result += numeric(lights, x - 1, y + 1);
    result += numeric(lights, x, y - 1);
    result += numeric(lights, x, y + 1);
    result += numeric(lights, x + 1, y - 1);
    result += numeric(lights, x + 1, y);
    result += numeric(lights, x + 1, y + 1);
    result
}

fn numeric(lights: &[LightRow], x: isize, y: isize) -> u32 {
    let max_x = lights.len();
    let max_y = lights[0].len();

    if x < 0 || y < 0 || max_x <= x as usize || max_y <= y as usize {
        0
    } else {
        u32::from(lights[x as usize][y as usize])
    }
}

fn is_corner(x: usize, y: usize, lights: &[LightRow]) -> bool {
    let max_x = lights.len() - 1;
    let max_y = lights[0].len() - 1;

    (x == 0 || x == max_x) && (y == 0 || y == max_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = ".#.#.#
    ...##.
    #....#
    ..#...
    #.#..#
    ####..";

    const INPUT_2: &str = "##.#.#
    ...##.
    #....#
    ..#...
    #.#..#
    ####.#";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&parse(INPUT_1), 4), 4);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(&parse(INPUT_2), 5), 17);
    }
}

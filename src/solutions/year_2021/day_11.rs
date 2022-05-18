pub fn solve(input: &str) {
    let mut energie_levels = parse(input);
    println!("Part 1: {}", part_one(&mut energie_levels.clone()));
    println!("Part 2: {}", part_two(&mut energie_levels));
}

const STEPS: usize = 100;
const FLASH: u8 = 9;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect()
        })
        .filter(|x: &Vec<u8>| !x.is_empty())
        .collect()
}

fn part_one(energie_levels: &mut Vec<Vec<u8>>) -> u64 {
    let mut flashes = 0;
    for _ in 0..STEPS {
        flashes += apply_step(energie_levels);
    }
    flashes
}

fn part_two(energie_levels: &mut Vec<Vec<u8>>) -> u64 {
    let all_octopus = energie_levels.iter().flatten().count() as u64;

    let mut steps = 1;
    while apply_step(energie_levels) < all_octopus {
        steps += 1;
    }
    steps
}

fn apply_step(energie_levels: &mut Vec<Vec<u8>>) -> u64 {
    for x in 0..energie_levels.len() {
        for y in 0..energie_levels[x].len() {
            apply_step_at((x, y), energie_levels);
        }
    }

    let mut count = 0;
    for row in energie_levels {
        for level in row {
            if *level > FLASH {
                *level = 0;
                count += 1;
            }
        }
    }
    count
}

fn apply_step_at((x, y): (usize, usize), energie_levels: &mut Vec<Vec<u8>>) {
    let max_x = energie_levels.len() - 1;
    let max_y = energie_levels[0].len() - 1;
    energie_levels[x][y] += 1;

    if energie_levels[x][y] - 1 == FLASH {
        if 0 < y {
            apply_step_at((x, y - 1), energie_levels);
        }

        if y < max_y {
            apply_step_at((x, y + 1), energie_levels);
        }

        if 0 < x {
            apply_step_at((x - 1, y), energie_levels);

            if 0 < y {
                apply_step_at((x - 1, y - 1), energie_levels);
            }
            if y < max_y {
                apply_step_at((x - 1, y + 1), energie_levels);
            }
        }

        if x < max_x {
            apply_step_at((x + 1, y), energie_levels);

            if 0 < y {
                apply_step_at((x + 1, y - 1), energie_levels);
            }
            if y < max_y {
                apply_step_at((x + 1, y + 1), energie_levels);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&mut parse(INPUT)), 1656);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(&mut parse(INPUT)), 195);
    }
}

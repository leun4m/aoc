use regex::Regex;
use std::collections::HashSet;

pub fn solve(input: &str) {
    let instructions = parse(input);
    println!("Part 1: {}", part_one(&instructions));
    // println!(
    //     "Part 2: {}",
    //     part_two(
    //         Player::create_at(instructions.0),
    //         Player::create_at(instructions.1)
    //     )
    // );
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: &str) -> Instruction {
    let regex = Regex::new(
        r"^(on|off) x=([-]?\d+)..([-]?\d+),y=([-]?\d+)..([-]?\d+),z=([-]?\d+)..([-]?\d+)$",
    )
    .unwrap();

    let captures = regex.captures(line.trim()).unwrap();
    let on = captures[1].trim() == "on";
    let x = parse_range(captures[2].trim(), captures[3].trim());
    let y = parse_range(captures[4].trim(), captures[5].trim());
    let z = parse_range(captures[6].trim(), captures[7].trim());

    Instruction { on, x, y, z }
}

fn parse_range(a: &str, b: &str) -> Range {
    Range {
        min: a.parse().unwrap(),
        max: b.parse().unwrap(),
    }
}

fn part_one(instructions: &[Instruction]) -> usize {
    let mut cuboid = Cuboid::new();
    for (i, instruction) in instructions.iter().enumerate() {
        cuboid.apply(instruction);
        log::trace!("{}", i);
    }
    cuboid.count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Range {
    min: i32,
    max: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Instruction {
    on: bool,
    x: Range,
    y: Range,
    z: Range,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cuboid {
    switched_on: HashSet<(i32, i32, i32)>,
}

impl Cuboid {
    fn new() -> Self {
        Self {
            switched_on: HashSet::new(),
        }
    }
    fn apply(&mut self, instruction: &Instruction) {
        if instruction.on {
            self.switch_on(instruction);
        } else {
            self.switch_off(instruction);
        }
    }

    fn switch_on(&mut self, instruction: &Instruction) {
        for x in instruction.x.min..=instruction.x.max {
            for y in instruction.y.min..=instruction.y.max {
                for z in instruction.z.min..=instruction.z.max {
                    self.switched_on.insert((x, y, z));
                }
            }
        }
    }

    fn switch_off(&mut self, instruction: &Instruction) {
        for x in instruction.x.min..=instruction.x.max {
            for y in instruction.y.min..=instruction.y.max {
                for z in instruction.z.min..=instruction.z.max {
                    self.switched_on.remove(&(x, y, z));
                }
            }
        }
    }

    fn count(&self) -> usize {
        self.switched_on.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "on x=10..12,y=10..12,z=10..12
    on x=11..13,y=11..13,z=11..13
    off x=9..11,y=9..11,z=9..11
    on x=10..10,y=10..10,z=10..10";

    #[test]
    fn parse_line_works() {
        assert_eq!(
            parse_line("on x=-20..26,y=-36..17,z=-47..7"),
            Instruction {
                on: true,
                x: Range { min: -20, max: 26 },
                y: Range { min: -36, max: 17 },
                z: Range { min: -47, max: 7 }
            }
        )
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&parse(INPUT)), 39);
    }
}

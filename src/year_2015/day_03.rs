use std::collections::HashSet;
use std::iter::FromIterator;

pub fn main(input: &str) {
    let year1 = houses_visited(&get_positions(&input));
    let year2 = houses_visited(&get_positions_with_robot(&input));
    println!("Year 1: {}", year1);
    println!("Year 2: {}", year2);
}

fn houses_visited(positions: &[(i32, i32)]) -> u32 {
    let unique: HashSet<(i32, i32)> = HashSet::from_iter(positions.iter().cloned());
    unique.len() as u32
}

fn get_positions(input: &str) -> Vec<(i32, i32)> {
    if !input.is_ascii() {
        panic!("Input is not ascii");
    }

    let mut santa = Position(0, 0);
    let mut houses = Vec::new();
    houses.push(santa.into());

    for char in input.chars() {
        santa.move_it(char);
        houses.push(santa.into());
    }
    houses
}

fn get_positions_with_robot(input: &str) -> Vec<(i32, i32)> {
    if !input.is_ascii() {
        panic!("Input is not ascii");
    }

    let mut is_santa = true;
    let mut santa = Position(0, 0);
    let mut robot = Position(0, 0);
    let mut houses = Vec::new();
    houses.push((santa.0, santa.0));
    houses.push((robot.0, robot.0));

    for char in input.chars() {
        if is_santa {
            santa.move_it(char);
            houses.push(santa.into());
        } else {
            robot.move_it(char);
            houses.push(robot.into());
        }
        is_santa = !is_santa;
    }
    houses
}

#[derive(Copy, Clone, Debug)]
struct Position(i32, i32);

impl Position {
    fn go_north(&mut self) {
        self.0 += 1;
    }
    fn go_south(&mut self) {
        self.0 -= 1;
    }
    fn go_east(&mut self) {
        self.1 += 1;
    }
    fn go_west(&mut self) {
        self.1 -= 1;
    }
    fn move_it(&mut self, char: char) {
        match char {
            '^' => self.go_north(),
            '>' => self.go_east(),
            'v' => self.go_south(),
            '<' => self.go_west(),
            _ => panic!("Unexpected char: {}", char),
        };
    }
}

impl Into<(i32, i32)> for Position {
    fn into(self) -> (i32, i32) {
        (self.0, self.1)
    }
}

#[cfg(test)]
mod test {
    use super::{get_positions, get_positions_with_robot, houses_visited};

    #[test]
    fn examples() {
        let ex1 = vec![(0, 0), (0, 1)];
        let ex2 = vec![(0, 0), (1, 0), (1, 1), (0, 1), (0, 0)];
        let ex3 = vec![
            (0, 0),
            (1, 0),
            (0, 0),
            (1, 0),
            (0, 0),
            (1, 0),
            (0, 0),
            (1, 0),
            (0, 0),
            (1, 0),
            (0, 0),
        ];

        assert_eq!(ex1, get_positions(">"));
        assert_eq!(ex2, get_positions("^>v<"));
        assert_eq!(ex3, get_positions("^v^v^v^v^v"));

        assert_eq!(2, houses_visited(&ex1));
        assert_eq!(4, houses_visited(&ex2));
        assert_eq!(2, houses_visited(&ex3));
    }

    #[test]
    fn example2() {
        assert_eq!(3, houses_visited(&get_positions_with_robot("^v")));
        assert_eq!(3, houses_visited(&get_positions_with_robot("^>v<")));
        assert_eq!(11, houses_visited(&get_positions_with_robot("^v^v^v^v^v")));
    }
}

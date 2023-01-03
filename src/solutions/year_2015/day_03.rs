use std::collections::HashSet;

pub fn solve(input: &str) {
    let year1 = houses_visited(&get_positions(input));
    let year2 = houses_visited(&get_positions_with_robot(input));
    println!("Year 1: {year1}");
    println!("Year 2: {year2}");
}

fn houses_visited(positions: &[(i32, i32)]) -> u32 {
    let unique: HashSet<(i32, i32)> = positions.iter().copied().collect::<HashSet<_, _>>();
    unique.len() as u32
}

fn get_positions(input: &str) -> Vec<(i32, i32)> {
    let mut santa = Position(0, 0);
    let mut houses = vec![santa.into()];

    for chr in input.chars() {
        santa.move_it(chr);
        houses.push(santa.into());
    }
    houses
}

fn get_positions_with_robot(input: &str) -> Vec<(i32, i32)> {
    let mut is_santa = true;
    let mut santa = Position(0, 0);
    let mut robot = Position(0, 0);
    let mut houses = vec![(santa.0, santa.0), (robot.0, robot.0)];

    for chr in input.chars() {
        if is_santa {
            santa.move_it(chr);
            houses.push(santa.into());
        } else {
            robot.move_it(chr);
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
    fn move_it(&mut self, c: char) {
        match c {
            '^' => self.go_north(),
            '>' => self.go_east(),
            'v' => self.go_south(),
            '<' => self.go_west(),
            _ => panic!("Unexpected char: {c}"),
        };
    }
}

impl From<Position> for (i32, i32) {
    fn from(position: Position) -> (i32, i32) {
        (position.0, position.1)
    }
}

#[cfg(test)]
mod tests {
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

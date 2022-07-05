use crate::parser;

pub fn solve(input: &str) {
    let numbers = parser::parse_numbers(input);
    println!("Part 1: {}", part_one(&numbers));
    println!("Part 2: {}", part_two(&numbers));
}

fn part_one(masses: &[i32]) -> i32 {
    masses.into_iter().map(|mass| fuel_requirement(*mass)).sum()
}

fn part_two(masses: &[i32]) -> i32 {
    masses
        .into_iter()
        .map(|mass| fuel_requirement_recursive(*mass))
        .sum()
}

fn fuel_requirement(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_requirement_recursive(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;

    if fuel > 0 {
        fuel + fuel_requirement_recursive(fuel)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_requirement_works() {
        assert_eq!(fuel_requirement(12), 2);
        assert_eq!(fuel_requirement(14), 2);
        assert_eq!(fuel_requirement(1969), 654);
        assert_eq!(fuel_requirement(100756), 33583);
    }

    #[test]
    fn fuel_requirement_recursive_works() {
        assert_eq!(fuel_requirement_recursive(14), 2);
        assert_eq!(fuel_requirement_recursive(1969), 966);
        assert_eq!(fuel_requirement_recursive(100756), 50346);
    }
}

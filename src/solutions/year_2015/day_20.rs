use std::io::{stdout, Write};

pub fn solve(input: &str) {
    let aim = input.parse().unwrap();
    let one = part_one(aim);
    println!("Part 1: {one}");
    println!("Part 2: {}", part_two(aim, one));
}

fn part_one(aim: u64) -> u64 {
    for house in 1..u64::MAX {
        if presents_in_house(house) >= aim {
            return house;
        }
        if house % 100_000 == 0 {
            print!(".");
            let _ = stdout().flush();
        }
    }
    0
}

fn part_two(aim: u64, start: u64) -> u64 {
    for house in start..u64::MAX {
        if presents_in_house_new(house) >= aim {
            return house;
        }
        if house % 100_000 == 0 {
            print!(".");
            let _ = stdout().flush();
        }
    }
    0
}

fn presents_in_house(house: u64) -> u64 {
    (1..=house).filter(|i| house % i == 0).map(|x| x * 10).sum()
}

fn presents_in_house_new(house: u64) -> u64 {
    (1..=house)
        .filter(|i| house / i <= 50)
        .filter(|i| house % i == 0)
        .map(|x| x * 11)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presents_in_house() {
        assert_eq!(10, presents_in_house(1));
        assert_eq!(30, presents_in_house(2));
        assert_eq!(40, presents_in_house(3));
        assert_eq!(70, presents_in_house(4));
        assert_eq!(60, presents_in_house(5));
        assert_eq!(120, presents_in_house(6));
        assert_eq!(80, presents_in_house(7));
        assert_eq!(150, presents_in_house(8));
        assert_eq!(130, presents_in_house(9));
    }
}

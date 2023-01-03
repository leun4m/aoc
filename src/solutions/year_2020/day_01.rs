use crate::parser;

pub fn solve(input: &str) {
    let numbers = parser::lines_as_numbers(input);

    match find_2_numbers(&numbers) {
        None => println!("Result for Part 1 could not be found!"),
        Some(x) => println!("Result Part 1: {x}"),
    }
    match find_3_numbers(&numbers) {
        None => println!("Result for Part 2 could not be found!"),
        Some(x) => println!("Result Part 2: {x}"),
    }
}

const EXPECTED_SUM: u32 = 2020;

fn find_2_numbers(numbers: &[u32]) -> Option<u32> {
    for i in 0..(numbers.len() - 2) {
        for j in (i + 1)..(numbers.len() - 1) {
            if numbers[i] + numbers[j] == EXPECTED_SUM {
                return Some(numbers[i] * numbers[j]);
            }
        }
    }
    None
}

fn find_3_numbers(numbers: &[u32]) -> Option<u32> {
    for i in 0..(numbers.len() - 2) {
        for j in (i + 1)..(numbers.len() - 1) {
            for k in (j + 1)..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == EXPECTED_SUM {
                    return Some(numbers[i] * numbers[j] * numbers[k]);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_NUMBERS: [u32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn part_one_works() {
        assert_eq!(find_2_numbers(&INPUT_NUMBERS), Some(514_579));
    }

    #[test]
    fn part_two_works() {
        assert_eq!(find_3_numbers(&INPUT_NUMBERS), Some(241_861_950));
    }
}

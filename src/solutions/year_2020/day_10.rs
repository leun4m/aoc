use crate::parser;

pub fn solve(input: &str) {
    let adapters = parse_input(input);
    let (difference_1, difference_3) = calculate_differences(&adapters);
    let variations = calculate_variations(&adapters);

    println!("Part 1: {}", difference_1 * difference_3);
    println!("Part 2: {variations}");
}

fn calculate_variations(adapters: &[u64]) -> i64 {
    let mut product: i64 = 1;
    calculate_difference_vec(adapters)
        .split(|&x| x == 3)
        .for_each(|x| product *= variations(x.len()));
    product
}

/// Calculates the number of variations for a given count of one distances
fn variations(ones: usize) -> i64 {
    match ones {
        0 | 1 => 1,
        2 => 2,
        3 => 4,
        4 => 7,
        5 => 11,
        // 1/2 * x^2 + 1/2 * x + 1
        _ => panic!("unexpected number of 1s: {ones}"),
    }
}

fn calculate_difference_vec(adapters: &[u64]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut previous = 0;
    for adapter in adapters {
        let diff = adapter - previous;
        result.push(diff as u8);
        previous = *adapter;
    }
    result
}

fn calculate_differences(adapters: &[u64]) -> (i32, i32) {
    let mut difference_1 = 0;
    let mut difference_3 = 1;
    let mut previous = 0;
    for adapter in adapters {
        let diff = adapter - previous;
        if diff == 3 {
            difference_3 += 1;
        } else if diff == 1 {
            difference_1 += 1;
        }
        previous = *adapter;
    }
    (difference_1, difference_3)
}

fn parse_input(input: &str) -> Vec<u64> {
    let mut adapters = parser::lines_as_numbers(input);
    adapters.sort_unstable();
    adapters
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let parsed = parse_input("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4");

        assert_eq!(vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19], parsed);
        assert_eq!((7, 5), calculate_differences(&parsed));
        assert_eq!(8, calculate_variations(&parsed));
    }

    #[test]
    fn example_2() {
        let input = &[
            1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35,
            38, 39, 42, 45, 46, 47, 48, 49,
        ];

        assert_eq!((22, 10), calculate_differences(input));
        assert_eq!(19208, calculate_variations(input));
    }
}

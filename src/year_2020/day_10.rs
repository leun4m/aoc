pub fn main(input: &str) {
    let adapters = parse_input(input);
    let (difference_1, difference_3) = calculate_differences(&adapters);
    let variations = calculate_variations(&adapters);

    println!("Part 1: {}", difference_1 * difference_3);
    println!("Part 2: {}", variations);
}

fn calculate_variations(adapters: &Vec<u64>) -> i64 {
    let mut product: i64 = 1;
    calculate_difference_vec(&adapters)
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
        _ => panic!("unexpected number of 1s: {}", ones),
    }
}

fn calculate_difference_vec(adapters: &Vec<u64>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut previous = 0;
    for adapter in adapters {
        let diff = adapter - previous;
        result.push(diff as u8);
        previous = *adapter;
    }
    result
}

fn calculate_differences(adapters: &Vec<u64>) -> (i32, i32) {
    let mut difference_1 = 0;
    let mut difference_3 = 1;
    let mut previous = 0;
    for adapter in adapters {
        let diff = adapter - previous;
        if diff == 3 {
            difference_3 += 1
        } else if diff == 1 {
            difference_1 += 1
        }
        previous = *adapter;
    }
    (difference_1, difference_3)
}

fn parse_input(input: &str) -> Vec<u64> {
    let mut adapters = input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    adapters.sort();
    adapters
}

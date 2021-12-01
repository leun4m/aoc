pub fn solve(input: &str) {
    let mut numbers = Vec::new();
    for line in input.lines() {
        numbers.push(line.parse::<u32>().expect("line is NaN"));
    }
    match find_2_numbers(&numbers) {
        None => println!("Result for Part 1 could not be found!"),
        Some(x) => println!("Result Part 1: {}", x),
    }
    match find_3_numbers(&numbers) {
        None => println!("Result for Part 2 could not be found!"),
        Some(x) => println!("Result Part 2: {}", x),
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

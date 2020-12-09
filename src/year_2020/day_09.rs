const PREAMBLE: usize = 25;

pub fn main(input: &str) {
    let numbers = parse_lines(input);
    let part_one = find_wrong_line(&numbers);
    if let Some(number) = part_one {
        println!("PART ONE: {}", number);
        if let Some(part_two) = find_set(number, &numbers) {
            println!("PART TWO: {}", part_two);
        }
    }
}

fn parse_lines(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn find_wrong_line(numbers: &[u64]) -> Option<u64> {
    for (i, number) in numbers[PREAMBLE..].iter().enumerate() {
        if !is_any_sum(*number, &numbers[i..(i + PREAMBLE)]) {
            return Some(number.clone());
        }
    }
    None
}

fn is_any_sum(number: u64, numbers: &[u64]) -> bool {
    for (idx, a) in numbers.iter().enumerate() {
        if numbers.iter().skip(idx + 1).any(|x| a + x == number) {
            return true;
        }
    }
    false
}

fn find_set(number: u64, numbers: &[u64]) -> Option<u64> {
    let mut set = Vec::new();
    let mut sum = 0;
    let mut start_idx = 0;

    while start_idx < numbers.len() {
        let mut idx = start_idx;
        while sum < number && idx < numbers.len() {
            set.push(numbers[idx]);
            sum = set.iter().sum();
            idx += 1;
            if sum == number {
                return Some(set.iter().min().unwrap() + set.iter().max().unwrap());
            }
        }

        set.clear();
        sum = 0;
        start_idx += 1;
    }

    None
}

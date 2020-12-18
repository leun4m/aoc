use std::collections::HashMap;

pub fn main(input: &str) {
    let numbers: Vec<usize> = input.split(',').map(|x| x.parse().unwrap()).collect();
    println!("Part 1: {}", play(&numbers, 2020));
    println!("Part 1: {}", play(&numbers, 30000000));
}

fn play(start: &[usize], number_of_interest: usize) -> usize {
    let mut numbers = HashMap::new();
    for (i, num) in start.iter().enumerate() {
        numbers.insert(*num, i + 1);
    }

    let mut previous = *start.iter().last().expect("List must not be empty!");
    let mut pre_previous;
    for n in start.len()..number_of_interest {
        pre_previous = previous;
        previous = if let Some(x) = numbers.get(&previous) {
            n - x
        } else {
            0
        };
        numbers.insert(pre_previous, n);
    }

    previous
}

#[cfg(test)]
mod test {
    use crate::year_2020::day_15::play;

    #[test]
    fn example() {
        const FIRST: usize = 2020;

        assert_eq!(1, play(&[1, 3, 2], FIRST));
        assert_eq!(10, play(&[2, 1, 3], FIRST));
        assert_eq!(27, play(&[1, 2, 3], FIRST));
        assert_eq!(78, play(&[2, 3, 1], FIRST));
        assert_eq!(438, play(&[3, 2, 1], FIRST));
        assert_eq!(1836, play(&[3, 1, 2], FIRST));
    }
}

pub fn solve(input: &str) {
    let fish = parse(input);
    println!("Part 1: {}", simulate(&fish, 80));
    println!("Part 2: {}", simulate(&fish, 256));
}

fn parse(input: &str) -> Vec<u8> {
    input
        .split(',')
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

const FISH_RESET: u8 = 6;
const FISH_NEW: u8 = 8;

fn simulate(fish: &[u8], days: u32) -> usize {
    let mut current_fish: Vec<u8> = fish.iter().copied().collect();
    for _ in 0..days {
        pass_day(&mut current_fish);
    }
    current_fish.len()
}

fn pass_day(fish: &mut Vec<u8>) {
    let count_new_fish = fish.iter().filter(|&&x| x == 0).count();
    for f in fish.iter_mut() {
        *f = if *f > 0 { *f - 1 } else { FISH_RESET };
    }
    for _ in 0..count_new_fish {
        fish.push(FISH_NEW);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn parse_works() {
        let output = parse(INPUT);
        assert_eq!(output, vec![3, 4, 3, 1, 2])
    }

    #[test]
    fn iterate_works() {
        let mut a = vec![3, 4, 3, 1, 2];
        pass_day(&mut a);
        assert_eq!(a, vec![2, 3, 2, 0, 1]);

        let mut b = vec![2, 3, 2, 0, 1];
        pass_day(&mut b);
        assert_eq!(b, vec![1, 2, 1, 6, 0, 8]);

        let mut c = vec![
            0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 0, 1, 2, 2, 2, 3, 3, 4, 4, 5, 7, 8,
        ];
        pass_day(&mut c);
        assert_eq!(
            c,
            vec![6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8]
        );
    }

    #[test]
    fn part_one_works() {
        let parsed = parse(INPUT);
        assert_eq!(simulate(&parsed, 18), 26);
        assert_eq!(simulate(&parsed, 80), 5934);
    }

    #[test]
    #[ignore]
    fn part_two_works() {
        let parsed = parse(INPUT);
        assert_eq!(simulate(&parsed, 256), 26984457539);
    }
}

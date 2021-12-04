pub fn solve(input: &str) {
    let (numbers, bit_size) = parse(input);
    println!("Part 1: {}", part_one(&numbers, bit_size));
}

fn parse(input: &str) -> (Vec<u32>, usize) {
    (
        input
            .lines()
            .filter(|x| !x.is_empty())
            .map(|x| u32::from_str_radix(x, 2).unwrap())
            .collect(),
        input.lines().map(|x| x.len()).max().unwrap_or_default(),
    )
}

fn part_one(numbers: &[u32], bit_size: usize) -> u32 {
    let gamma = calc_gamma(numbers, bit_size);
    let epsilon = invert(gamma, bit_size);
    gamma * epsilon
}

fn calc_gamma(numbers: &[u32], bit_size: usize) -> u32 {
    let half = (numbers.len() + 1) / 2;
    let bit_at = |number: u32, position: u32| number >> position & 1;
    
    (0..(bit_size as u32))
        .collect::<Vec<u32>>()
        .iter()
        .map(|&i| {
            if numbers.iter().filter(|&&x| bit_at(x, i) == 0).count() <= half {
                2_u32.pow(i)
            } else {
                0
            }
        })
        .sum()
}

fn invert(number: u32, bit_size: usize) -> u32 {
    let inverted: String = format!("{:width$b}", number, width = bit_size)
        .chars()
        .map(|x| match x {
            '1' => '0',
            _ => '1',
        })
        .collect();
    u32::from_str_radix(&inverted, 2).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn parse_works() {
        assert_eq!(
            parse(INPUT),
            (vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10], 5)
        );
    }
    
    #[test]
    fn calc_most_common_works_simple() {
        assert_eq!(calc_gamma(&[1], 1), 1);
    }

    #[test]
    fn calc_most_common_works() {
        let numbers = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let bit_size = 5;
        assert_eq!(calc_gamma(&numbers, bit_size), 22);
    }

    #[test]
    fn part_one_works() {
        let numbers = vec![4, 30, 22, 23, 21, 15, 7, 28, 16, 25, 2, 10];
        let bit_size = 5;
        assert_eq!(part_one(&numbers, bit_size), 198);
    }
}

pub fn solve(input: &str) {
    let trimmed = input.trim();
    println!("{}", part_one(trimmed));
    println!("{}", part_two(trimmed));
}

fn part_one(input: &str) -> u32 {
    internal(input, 1)
}

fn part_two(input: &str) -> u32 {
    internal(input, input.len() / 2)
}

fn internal(input: &str, steps: usize) -> u32 {
    if input.len() < 2 {
        return 0;
    }

    let mut sum = 0;
    let chars = input.chars().collect::<Vec<char>>();

    for i in 0..input.len() {
        let a = chars[i];
        let b = chars[(i + steps) % input.len()];

        if a == b {
            sum += a.to_digit(10).unwrap();
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn part_one_works() {
        assert_eq!(part_one("1122"), 3);
        assert_eq!(part_one("1111"), 4);
        assert_eq!(part_one("1234"), 0);
        assert_eq!(part_one("91212129"), 9);
        assert_eq!(part_one("91212199"), 18);
    }

    #[test]
    pub fn part_two_works() {
        assert_eq!(part_two("1212"), 6);
        assert_eq!(part_two("1221"), 0);
        assert_eq!(part_two("123425"), 4);
        assert_eq!(part_two("123123"), 12);
        assert_eq!(part_two("12131415"), 4);
    }
}

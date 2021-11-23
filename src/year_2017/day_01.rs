pub fn main(input: &str) {
    let trimmed = input.trim();
    println!("{}", part_one(trimmed));
    // println!("{}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    if input.len() < 2 {
        return 0;
    }

    let mut sum = 0;
    let mut last_char = input.chars().last().unwrap();

    for c in input.chars() {
        if c == last_char {
            sum += c.to_digit(10).unwrap();
        }
        last_char = c;
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
}

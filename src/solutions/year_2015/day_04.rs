pub fn solve(input: &str) {
    println!("5 leading Zeros: {}", find_number(input, 5));
    println!("6 leading Zeros: {}", find_number(input, 6));
}

fn find_number(input: &str, leading_zeros: usize) -> u32 {
    let prefix = "0".repeat(leading_zeros);
    for i in 1.. {
        if md5::compute(format!("{}{}", input, &i)).starts_with(prefix.as_bytes()) {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::find_number;

    #[test]
    #[ignore]
    fn example() {
        assert_eq!(609043, find_number("abcdef", 5));
        assert_eq!(1048970, find_number("pqrstuv", 5));
        assert_eq!(9962624, find_number("yzbqklnj", 6));
    }
}

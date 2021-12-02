use crypto::digest::Digest;
use crypto::md5::Md5;

pub fn solve(input: &str) {
    println!("5 leading Zeros: {}", find_number(input, 5));
    println!("6 leading Zeros: {}", find_number(input, 6));
}

fn find_number(input: &str, leading_zeros: usize) -> u32 {
    let prefix = "0".repeat(leading_zeros);
    let mut md5 = Md5::new();
    md5.input_str(input);
    for i in 1..u32::MAX {
        let mut a = md5;
        a.input_str(&i.to_string());
        if a.result_str().starts_with(&prefix) {
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

pub fn main(input: &str) {
    println!("5 leading Zeros: {}", find_number(input, 5));
    println!("6 leading Zeros: {}", find_number(input, 6));
}

fn find_number(input: &str, leading_zeros: usize) -> u32 {
    let prefix = &"0".repeat(leading_zeros);
    for i in 1..u32::MAX {
        if format!("{:x}", md5::compute([input, &i.to_string()].concat())).starts_with(prefix) {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::find_number;

    #[test]
    fn example() {
        assert_eq!(609043, find_number("abcdef", 5));
        assert_eq!(1048970, find_number("pqrstuv", 5));
    }
}

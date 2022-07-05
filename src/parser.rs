use std::{fmt::Debug, str::FromStr};

/// Parses each non-empty line as number
pub fn parse_numbers<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_works() {
        assert_eq!(parse_numbers::<i32>("0\n+1\n\n\n-3"), vec![0, 1, -3]);
    }
}

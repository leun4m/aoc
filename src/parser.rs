use std::{fmt::Debug, str::FromStr};

/// Parses each non-empty line as number
pub fn lines_as_numbers<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    lines_custom(input, |line| line.parse().unwrap())
}

/// Interprets each non-empty line as independent string
pub fn lines_as_strings(input: &str) -> Vec<&str> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect()
}

/// Performs mapping on each non-empty line
pub fn lines_custom<T, F>(input: &str, parse_line: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numbers_works() {
        assert_eq!(lines_as_numbers::<i32>("0\n+1\n\n\n-3"), vec![0, 1, -3]);
    }
}

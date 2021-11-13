use std::collections::HashMap;

pub const INVALID_DAY: &str = "There exists no implementation for this day";
pub const INVALID_YEAR: &str = "Not a valid year!";
pub const NO_INPUT: &str = "No input given!";

/// Counts occurences of every char in `text`.
pub fn count_chars(text: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();

    for c in text.chars() {
        let counter = map.get(&c).copied().unwrap_or(0);
        map.insert(c, counter + 1);
    }

    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_chars_works() {
        assert_eq!(count_chars(""), HashMap::new());
        assert_eq!(count_chars("a"), HashMap::from([('a', 1)]));
        assert_eq!(
            count_chars("abacab"),
            HashMap::from([('a', 3), ('b', 2), ('c', 1)])
        );
    }
}

use std::fmt::Debug;
use std::collections::HashMap;

pub const INVALID_DAY: &str = "There exists no implementation for this day";
pub const INVALID_YEAR: &str = "Not a valid year!";
pub const NO_INPUT: &str = "No input given!";

/// Counts occurences of every char in `text`.
pub fn count_chars(text: &str) -> HashMap<char, u32> {
    let mut result = HashMap::new();

    for c in text.chars() {
        *result.entry(c).or_insert(0) += 1;
    }

    result
}

pub fn permutation_heap<T: Clone + Debug>(elements: &mut Vec<T>) -> Vec<Vec<T>> {
    let mut generated_permutations = vec![Vec::from(elements.as_slice())];

    let mut c = vec![0; elements.len()];
    let mut i = 0;
    while i < elements.len() {
        if c[i] < i {
            if i % 2 == 0 {
                elements.swap(0, i);
            } else {
                elements.swap(c[i], i);
            }
            generated_permutations.push(Vec::from(elements.as_slice()));
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    generated_permutations
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

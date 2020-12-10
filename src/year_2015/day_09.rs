use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

pub fn main(input: &str) {
    let result = internal(input);
    println!("Shortest: {}", result.0);
    println!("Longest:  {}", result.1);
}

fn internal(input: &str) -> (u64, u64) {
    let mut connections = HashMap::new();
    for line in input.lines() {
        let (from, to, distance) = parse_line(line);
        connections.insert((from.clone(), to.clone()), distance);
        connections.insert((to, from), distance);
    }

    let permutations = permutation_heap(&mut get_cities(&connections));
    println!("PERMUTATIONS: {}", permutations.len());
    shortest_longest(&permutations, &connections)
}

fn shortest_longest(
    permutations: &[Vec<String>],
    connections: &HashMap<(String, String), u64>,
) -> (u64, u64) {
    let mut min_duration = u64::MAX;
    let mut max_duration = u64::MIN;

    for permutation in permutations {
        let mut duration = 0;
        let mut i = 1;
        let mut is_valid_path = true;
        while i < permutation.len() && is_valid_path {
            match connections.get(&(permutation[i - 1].clone(), permutation[i].clone())) {
                None => is_valid_path = false,
                Some(x) => duration += x,
            }
            i += 1;
        }
        if duration < min_duration {
            min_duration = duration;
        }
        if duration > max_duration {
            max_duration = duration;
        }
    }

    (min_duration, max_duration)
}

fn get_cities(connections: &HashMap<(String, String), u64>) -> Vec<String> {
    let mut cities = HashSet::new();
    connections.keys().for_each(|(a, b)| {
        cities.insert(a.clone());
        cities.insert(b.clone());
    });
    cities.into_iter().collect()
}

fn permutation_heap<T: Clone + Debug>(elements: &mut Vec<T>) -> Vec<Vec<T>> {
    let mut generated_permutations = Vec::new();
    generated_permutations.push(Vec::from(elements.as_slice()));

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

fn parse_line(line: &str) -> (String, String, u64) {
    let regex = Regex::new(r"^(\w+) to (\w+) = (\d+)$").unwrap();
    let captures = regex
        .captures(line)
        .unwrap_or_else(|| panic!("Line doesn't look as expected: {}", line));
    (
        captures.get(1).unwrap().as_str().to_string(),
        captures.get(2).unwrap().as_str().to_string(),
        captures.get(3).unwrap().as_str().parse::<u64>().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::internal;

    #[test]
    fn example() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";
        let result = internal(input);
        assert_eq!(605, result.0);
        assert_eq!(982, result.1);
    }
}

use std::collections::HashSet;

pub fn main(input: &str) {
    let groups = parse_input(input);

    let sum_1 = part_one(&groups);
    let sum_2 = part_two(&groups);

    println!("Sum 1: {}", sum_1);
    println!("Sum 2: {}", sum_2);
}

fn part_one(groups: &Vec<Vec<Vec<char>>>) -> usize {
    let mut group_answers: Vec<HashSet<&char>> = Vec::new();
    for group in groups {
        let mut group_set = HashSet::new();
        for person in group {
            for answer in person {
                group_set.insert(answer);
            }
        }
        group_answers.push(group_set.clone());
        group_set.clear();
    }

    let mut sum = 0;
    for a in &group_answers {
        sum += a.len();
    }
    sum
}

fn part_two(groups: &Vec<Vec<Vec<char>>>) -> usize {
    let mut group_answers = Vec::new();
    for group in groups {
        let mut group_set = Vec::new();
        let mut first = true;
        for person in group {
            if first {
                first = false;
                for c in person {
                    group_set.push(c);
                }
            } else {
                group_set = group_set
                    .iter()
                    .filter(|x| person.contains(*x))
                    .map(|x| *x)
                    .collect();
            }
        }
        group_answers.push(group_set.clone());
        group_set.clear();
    }

    let mut sum = 0;
    for a in &group_answers {
        sum += a.len();
    }
    sum
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            groups.push(group.clone());
            group.clear();
        } else {
            let person: Vec<char> = line.chars().collect();
            group.push(person);
        }
    }
    groups.push(group.clone());
    groups
}

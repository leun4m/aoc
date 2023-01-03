use std::collections::HashSet;

pub fn solve(input: &str) {
    let groups = parse_input(input);

    let sum_1 = part_one(&groups);
    let sum_2 = part_two(&groups);

    println!("Sum 1: {sum_1}");
    println!("Sum 2: {sum_2}");
}

fn part_one(groups: &[Vec<Vec<char>>]) -> usize {
    groups
        .iter()
        .map(|group| group.iter().flatten().collect::<HashSet<&char>>().len())
        .sum()
}

fn part_two(groups: &[Vec<Vec<char>>]) -> usize {
    let mut sum = 0;
    for group in groups {
        let mut group_set = Vec::new();
        let mut first_person = true;
        for person in group {
            if first_person {
                first_person = false;
                group_set = person.iter().collect();
            } else {
                group_set.retain(|x| person.contains(*x));
            }
        }
        sum += group_set.len();
        group_set.clear();
    }

    sum
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|x| x.split('\n').map(|x| x.chars().collect()).collect())
        .collect()
}

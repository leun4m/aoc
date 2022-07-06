use crate::parser;

pub fn solve(input: &str) {
    let jumps: Vec<i32> = parser::lines_as_numbers(input);
    println!("Part 1: {}", part_one(&jumps));
    println!("Part 2: {}", part_two(&jumps));
}

fn part_one(offsets: &[i32]) -> u32 {
    process(offsets, |x| x + 1)
}

fn part_two(offsets: &[i32]) -> u32 {
    process(offsets, |x| if x >= 3 { x - 1 } else { x + 1 })
}

fn process<F>(offsets: &[i32], new_val: F) -> u32
where
    F: Fn(i32) -> i32,
{
    let mut jumps: Vec<i32> = offsets.into();
    let mut steps = 0;
    let mut index = 0;

    while let Some(offset) = jumps.get_mut(index as usize) {
        index += *offset;
        *offset = new_val(*offset);

        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        let input = vec![0, 3, 0, 1, -3];
        assert_eq!(part_one(&input), 5);
    }

    #[test]
    fn part_two_works() {
        let input = vec![0, 3, 0, 1, -3];
        assert_eq!(part_two(&input), 10);
    }
}

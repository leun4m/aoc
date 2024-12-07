use crate::parser;

pub fn solve(input: &str) {
    let reports = parse(input);
    println!("Part 1: {}", part_one(&reports));
    println!("Part 2: {}", part_two(&reports));
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    parser::lines_custom(input, |line| {
        line.split(' ')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>()
    })
}

fn part_one(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|&x| is_report_safe(x)).count()
}

fn part_two(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|&report| {
            is_report_safe(report)
                || report.iter().enumerate().any(|(j, _)| {
                    let mut report_without_j = report.clone();
                    report_without_j.remove(j);
                    is_report_safe(&report_without_j)
                })
        })
        .count()
}

fn is_report_safe(report: &[i32]) -> bool {
    report.len() >= 2
        && report.windows(2).all(|x| {
            let diff = i32::abs(x[0] - x[1]);
            0 < diff && diff <= 3
        })
        && (report.windows(2).all(|x| x[0] <= x[1]) || report.windows(2).all(|x| x[0] >= x[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_one() {
        assert_eq!(2, part_one(&parse(EXAMPLE_INPUT)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(4, part_two(&parse(EXAMPLE_INPUT)));
    }
}

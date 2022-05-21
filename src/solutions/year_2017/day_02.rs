use itertools::Itertools;

pub fn solve(input: &str) {
    let table = parse(input);
    println!("Part 1: {}", checksum_one(&table));
    println!("Part 2: {}", checksum_two(&table));
}

type Table = Vec<Row>;
type Row = Vec<Base>;
type Base = i32;

fn parse(input: &str) -> Table {
    input
        .lines()
        .map(|row| row.trim())
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.split_ascii_whitespace()
                .map(|cell| cell.parse::<Base>().expect("NaN"))
                .collect_vec()
        })
        .collect_vec()
}

fn checksum_one(table: &[Row]) -> Base {
    table.iter().map(|row| checksum_row(row)).sum()
}

fn checksum_two(table: &[Row]) -> Base {
    table.iter().map(|row| pure_division(row)).sum()
}

fn checksum_row(row: &[Base]) -> Base {
    row.iter().max().unwrap_or(&0) - row.iter().min().unwrap_or(&0)
}

fn pure_division(row: &[Base]) -> Base {
    for i in 0..row.len() {
        for k in i + 1..row.len() {
            if row[i] % row[k] == 0 {
                return row[i] / row[k];
            } else if row[k] % row[i] == 0 {
                return row[k] / row[i];
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_row_works() {
        assert_eq!(checksum_row(&[5, 1, 9, 5]), 8);
        assert_eq!(checksum_row(&[7, 5, 3]), 4);
        assert_eq!(checksum_row(&[2, 4, 6, 8]), 6);
    }

    #[test]
    fn checksum_one_works() {
        assert_eq!(
            checksum_one(&[vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]]),
            18
        );
    }

    #[test]
    fn pure_division_works() {
        assert_eq!(pure_division(&[5, 9, 2, 8]), 4);
        assert_eq!(pure_division(&[9, 4, 7, 3]), 3);
        assert_eq!(pure_division(&[3, 8, 6, 5]), 2);
    }

    #[test]
    fn checksum_two_works() {
        assert_eq!(
            checksum_two(&[vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]]),
            9
        );
    }
}

use itertools::Itertools;

pub fn solve(input: &str) {
    let table = parse(input);
    println!("Part 1: {}", checksum(&table));
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

fn checksum(table: &[Row]) -> Base {
    table.iter().map(|row| checksum_row(row)).sum()
}

fn checksum_row(row: &[Base]) -> Base {
    row.iter().max().unwrap_or(&0) - row.iter().min().unwrap_or(&0)
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
    fn checksum_works() {
        assert_eq!(
            checksum(&[vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]]),
            18
        );
    }
}

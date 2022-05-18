pub fn solve(input: &str) {
    let (drafts, boards) = parse(input);
    println!("Part 1: {}", part_one(&drafts, &boards));
    println!("Part 2: {}", part_two(&drafts, &boards));
}

const BOARD_SIZE: usize = 5;

type BingoNumber = u32;
type BingoRow = [BingoNumber; BOARD_SIZE];
type BingoBoard = [BingoRow; BOARD_SIZE];

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum BingoSquare {
    Marked(BingoNumber),
    Unmarked(BingoNumber),
}

type MarkedBingoRow = [BingoSquare; BOARD_SIZE];
type MarkedBingoBoard = [[BingoSquare; BOARD_SIZE]; BOARD_SIZE];

fn parse(input: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    (parse_drafts(input), parse_boards(input))
}

fn parse_drafts(input: &str) -> Vec<u32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_boards(input: &str) -> Vec<BingoBoard> {
    input
        .lines()
        .skip(1)
        .filter(|x| !x.trim().is_empty())
        .collect::<Vec<&str>>()
        .chunks_exact(BOARD_SIZE)
        .map(parse_board)
        .collect()
}

fn parse_board(board: &[&str]) -> BingoBoard {
    board
        .iter()
        .map(|row| parse_row(row))
        .collect::<Vec<BingoRow>>()
        .try_into()
        .unwrap()
}

fn parse_row(row: &str) -> BingoRow {
    row.split(' ')
        .filter(|x| !x.trim().is_empty())
        .map(|num| num.parse().unwrap())
        .collect::<Vec<BingoNumber>>()
        .try_into()
        .unwrap()
}

fn to_marked_board(board: &BingoBoard) -> MarkedBingoBoard {
    board
        .iter()
        .map(|row| {
            row.iter()
                .copied()
                .map(BingoSquare::Unmarked)
                .collect::<Vec<BingoSquare>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<MarkedBingoRow>>()
        .try_into()
        .unwrap()
}

fn part_one(drafts: &[BingoNumber], boards: &[BingoBoard]) -> BingoNumber {
    let mut marked_boards: Vec<MarkedBingoBoard> =
        boards.iter().map(to_marked_board).collect();

    for draft in drafts {
        for board in marked_boards.iter_mut() {
            mark_board(board, *draft);
        }
        let winner = marked_boards.iter().find(|board| has_won(board));
        if let Some(w) = winner {
            return sum_up(w) * draft;
        }
    }
    0
}

fn part_two(drafts: &[BingoNumber], boards: &[BingoBoard]) -> BingoNumber {
    let mut marked_boards: Vec<MarkedBingoBoard> =
        boards.iter().map(to_marked_board).collect();

    for draft in drafts {
        for board in marked_boards.iter_mut() {
            mark_board(board, *draft);
        }

        if marked_boards.len() == 1 {
            let winner = marked_boards.iter().find(|board| has_won(board));
            if let Some(w) = winner {
                return sum_up(w) * draft;
            }
        }

        marked_boards.retain(|board| !has_won(board));
    }
    0
}

fn mark_board(board: &mut MarkedBingoBoard, draft: BingoNumber) {
    for row in board.iter_mut() {
        for square in row.iter_mut() {
            if let BingoSquare::Unmarked(num) = &square {
                if *num == draft {
                    *square = BingoSquare::Marked(draft)
                }
            }
        }
    }
}

fn has_won(board: &MarkedBingoBoard) -> bool {
    let has_col = |b: &MarkedBingoBoard, x: usize, y: usize| b[x][y];
    let has_row = |b: &MarkedBingoBoard, x: usize, y: usize| b[y][x];

    check_board(board, has_col) || check_board(board, has_row)
}

fn check_board<F>(board: &MarkedBingoBoard, f: F) -> bool
where
    F: Fn(&MarkedBingoBoard, usize, usize) -> BingoSquare,
{
    let indices: Vec<_> = (0..BOARD_SIZE).collect();

    indices.iter().any(|&x| {
        indices
            .iter()
            .all(|&y| matches!(f(board, x, y), BingoSquare::Marked(_)))
    })
}

fn sum_up(board: &MarkedBingoBoard) -> BingoNumber {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|num| match num {
                    BingoSquare::Unmarked(x) => *x,
                    _ => 0,
                })
                .sum::<BingoNumber>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7    
";

    const DRAFTS: [BingoNumber; 27] = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    const BOARDS: [BingoBoard; 3] = [
        [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ],
        [
            [3, 15, 0, 2, 22],
            [9, 18, 13, 17, 5],
            [19, 8, 7, 25, 23],
            [20, 11, 10, 24, 4],
            [14, 21, 16, 12, 6],
        ],
        [
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ],
    ];

    #[test]
    fn parse_works() {
        let (drafts, boards) = parse(INPUT);
        assert_eq!(&drafts, &DRAFTS);
        assert_eq!(&boards, &BOARDS);
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&DRAFTS, &BOARDS), 4512);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(&DRAFTS, &BOARDS), 1924);
    }
}

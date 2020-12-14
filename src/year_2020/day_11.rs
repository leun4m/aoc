const PART_ONE_SEAT_RULE: usize = 4;
const PART_TWO_SEAT_RULE: usize = 5;

pub fn main(input: &str) {
    let start_positions = parse_input(input);

    println!("Part 1: {}", part_one(&start_positions));
    println!("Part 2: {}", part_two(&start_positions));
}

fn part_one(input: &[Vec<Position>]) -> usize {
    let mut seats_before = input.to_vec();

    loop {
        let seats_after = calc_new_seats(&seats_before, get_adjacent_1, PART_ONE_SEAT_RULE);

        if seats_before == seats_after {
            break;
        }

        seats_before = seats_after;
    }

    sum_occupied(&seats_before)
}

fn part_two(input: &[Vec<Position>]) -> usize {
    let mut seats_before = input.to_vec();

    loop {
        let seats_after = calc_new_seats(&seats_before, get_adjacent_2, PART_TWO_SEAT_RULE);

        if seats_before == seats_after {
            break;
        }

        seats_before = seats_after;
    }

    sum_occupied(&seats_before)
}

fn sum_occupied(matrix: &[Vec<Position>]) -> usize {
    matrix
        .iter()
        .flatten()
        .filter(|x| **x == Position::SeatOccupied)
        .count()
}

fn calc_new_seats<F>(old_seats: &[Vec<Position>], adjacent: F, number: usize) -> Vec<Vec<Position>>
where
    F: Fn(&[Vec<Position>], usize, usize) -> Vec<Position>,
{
    let mut new_seats = Vec::new();

    for (x, row) in old_seats.iter().enumerate() {
        let mut new_row = Vec::new();
        for (y, seat) in row.iter().enumerate() {
            let new_seat = match seat {
                Position::SeatEmpty => {
                    if adjacent(old_seats, x, y)
                        .iter()
                        .all(|&x| x != Position::SeatOccupied)
                    {
                        Position::SeatOccupied
                    } else {
                        *seat
                    }
                }
                Position::SeatOccupied => {
                    if adjacent(old_seats, x, y)
                        .iter()
                        .filter(|&&x| x == Position::SeatOccupied)
                        .count()
                        >= number
                    {
                        Position::SeatEmpty
                    } else {
                        *seat
                    }
                }
                _ => *seat,
            };
            new_row.push(new_seat);
        }
        new_seats.push(new_row);
    }

    new_seats
}

fn get_adjacent_1(seats: &[Vec<Position>], x: usize, y: usize) -> Vec<Position> {
    let mut result = Vec::new();
    let x_len = seats.len();
    let y_len = seats[0].len();

    if x > 0 {
        // TOP
        result.push(seats[x - 1][y]);

        // LEFT TOP
        if y > 0 {
            result.push(seats[x - 1][y - 1]);
        }

        // RIGHT TOP
        if y + 1 < y_len {
            result.push(seats[x - 1][y + 1]);
        }
    }

    // LEFT
    if y > 0 {
        result.push(seats[x][y - 1]);
    }

    // RIGHT
    if y + 1 < y_len {
        result.push(seats[x][y + 1]);
    }

    if x + 1 < x_len {
        // DOWN
        result.push(seats[x + 1][y]);

        // LEFT DOWN
        if y > 0 {
            result.push(seats[x + 1][y - 1]);
        }

        // RIGHT DOWN
        if y + 1 < y_len {
            result.push(seats[x + 1][y + 1]);
        }
    }

    result
}

fn get_adjacent_2(seats: &[Vec<Position>], x_o: usize, y_p: usize) -> Vec<Position> {
    let mut result = Vec::new();
    let x_len = seats.len();
    let y_len = seats[0].len();

    let inc = |x| x + 1;
    let dec = |x| x - 1;
    let id = |x| x;

    // TOP
    result.push(get_adjacent_at(seats, x_o, y_p, dec, id, |x, _| x > 0));

    // TOP LEFT
    result.push(get_adjacent_at(seats, x_o, y_p, dec, dec, |a, b| {
        a > 0 && b > 0
    }));

    // TOP RIGHT
    result.push(get_adjacent_at(seats, x_o, y_p, dec, inc, |x, y| {
        x > 0 && y + 1 < y_len
    }));

    // MIDDLE LEFT
    result.push(get_adjacent_at(seats, x_o, y_p, id, dec, |_, y| y > 0));

    // MIDDLE RIGHT
    result.push(get_adjacent_at(seats, x_o, y_p, id, inc, |_, y| {
        y + 1 < y_len
    }));

    // DOWN
    result.push(get_adjacent_at(seats, x_o, y_p, inc, id, |x, _| {
        x + 1 < x_len
    }));

    // DOWN LEFT
    result.push(get_adjacent_at(seats, x_o, y_p, inc, dec, |x, y| {
        x + 1 < x_len && y > 0
    }));

    // DOWN RIGHT
    result.push(get_adjacent_at(seats, x_o, y_p, inc, inc, |x, y| {
        x + 1 < x_len && y + 1 < y_len
    }));

    result
}

fn get_adjacent_at<F, G, H>(
    seats: &[Vec<Position>],
    x: usize,
    y: usize,
    x_inc: F,
    y_inc: G,
    cmp: H,
) -> Position
where
    F: Fn(usize) -> usize,
    G: Fn(usize) -> usize,
    H: Fn(usize, usize) -> bool,
{
    if !cmp(x, y) {
        return Position::Floor;
    }

    let mut x_ptr = x_inc(x);
    let mut y_ptr = y_inc(y);

    if cmp(x_ptr, y_ptr) {
        while cmp(x_ptr, y_ptr) && seats[x_ptr][y_ptr] == Position::Floor {
            y_ptr = y_inc(y_ptr);
            x_ptr = x_inc(x_ptr);
        }
    }

    seats[x_ptr][y_ptr]
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Position {
    Floor,
    SeatEmpty,
    SeatOccupied,
}

fn parse_input(input: &str) -> Vec<Vec<Position>> {
    let mut result = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();

        for char in line.chars() {
            row.push(match char {
                '.' => Position::Floor,
                'L' => Position::SeatEmpty,
                '#' => Position::SeatOccupied,
                _ => panic!("Unexpected symbol: {}", char),
            })
        }

        result.push(row);
    }

    result
}

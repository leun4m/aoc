use regex::Regex;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

type Matrix<T> = Vec<Vec<T>>;

pub fn solve(input: &str) {
    let result = main_internal(input);
    println!("Lights on:  {}", result.0);
    println!("Brightness: {}", result.1);
}

fn main_internal(input: &str) -> (u32, u32) {
    let mut io_matrix = create_2d_matrix(WIDTH, HEIGHT, false);
    let mut led_matrix = create_2d_matrix(WIDTH, HEIGHT, 0);
    for line in input.lines() {
        let regex = Regex::new(r"^(.+) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
        let cap = regex
            .captures(line)
            .unwrap_or_else(|| panic!("Invalid cmd: {}", &line));
        let cmd = cap.get(1).unwrap().as_str();
        let start = (
            cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        );
        let stop = (
            cap.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            cap.get(5).unwrap().as_str().parse::<usize>().unwrap(),
        );
        perform(&mut io_matrix, start, stop, get_io_operation(cmd));
        perform(&mut led_matrix, start, stop, get_led_operation(cmd));
    }
    (count_lit_lights(&io_matrix), sum_brightness(&led_matrix))
}

fn sum_brightness(matrix: &[Vec<u32>]) -> u32 {
    let mut brightness = 0;
    for row in matrix.iter() {
        for led in row.iter() {
            brightness += led;
        }
    }
    brightness
}

fn count_lit_lights(matrix: &[Vec<bool>]) -> u32 {
    let mut lights_on = 0;
    for row in matrix.iter() {
        for led in row.iter() {
            if *led {
                lights_on += 1;
            }
        }
    }
    lights_on
}

fn get_io_operation(cmd: &str) -> fn(bool) -> bool {
    match cmd {
        "toggle" => |x| !x,
        "turn off" => |_| false,
        "turn on" => |_| true,
        _ => |x| x,
    }
}

fn get_led_operation(cmd: &str) -> fn(u32) -> u32 {
    match cmd {
        "toggle" => |x| x + 2,
        "turn off" => |x| if x > 0 { x - 1 } else { 0 },
        "turn on" => |x| x + 1,
        _ => |x| x,
    }
}

fn perform<F, T>(matrix: &mut Matrix<T>, start: (usize, usize), stop: (usize, usize), f: F)
where
    F: Fn(T) -> T,
    T: Copy,
{
    for row in matrix.iter_mut().take(stop.0 + 1).skip(start.0) {
        for cell in row.iter_mut().take(stop.1 + 1).skip(start.1) {
            *cell = f(*cell);
        }
    }
}

fn create_2d_matrix<T>(width: usize, height: usize, default: T) -> Matrix<T>
where
    T: Copy,
{
    let mut matrix = Vec::new();
    for _ in 0..width {
        let mut row = Vec::new();
        for _ in 0..height {
            row.push(default);
        }
        matrix.push(row);
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::{main_internal, HEIGHT, WIDTH};

    const ALL: u32 = (HEIGHT * WIDTH) as u32;

    #[test]
    fn example_empty() {
        let result = main_internal("");
        assert_eq!(0, result.0);
        assert_eq!(0, result.1);
    }

    #[test]
    fn example_1() {
        let result = main_internal("turn on 0,0 through 999,999");
        assert_eq!(ALL, result.0);
        assert_eq!(ALL, result.1);
    }

    #[test]
    fn example_2() {
        let result = main_internal("toggle 0,0 through 999,0");
        assert_eq!(1000, result.0);
        assert_eq!(2000, result.1);
    }

    #[test]
    fn example_3() {
        let result = main_internal("turn on 0,0 through 999,999\nturn off 499,499 through 500,500");
        assert_eq!(ALL - 4, result.0);
        assert_eq!(ALL - 4, result.1);
    }

    #[test]
    fn example_4() {
        let result = main_internal("turn on 0,0 through 0,0\nturn on 0,0 through 0,0");
        assert_eq!(1, result.0);
        assert_eq!(2, result.1);
    }

    #[test]
    fn example_5() {
        let result = main_internal("toggle 0,0 through 999,999");
        assert_eq!(ALL, result.0);
        assert_eq!(ALL * 2, result.1);
    }
}

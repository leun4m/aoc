pub fn solve(input: &str) {
    if !input.is_ascii() {
        panic!("Input is not ascii!");
    }
    let (floor, index) = count(input);
    println!("Floor: {}", floor);
    println!("First time in basement: {}", index);
}

fn count(input: &str) -> (i32, i32) {
    let mut floor = 0;
    let mut index = 1;
    let mut reached_basement = false;
    for chr in input.chars() {
        match chr {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Unexpected char: {}", chr),
        }
        if floor == -1 {
            reached_basement = true;
        } else if !reached_basement {
            index += 1;
        }
    }
    if !reached_basement {
        index = -1;
    }
    (floor, index)
}

#[cfg(test)]
mod tests {
    use crate::year_2015::day_01::count;

    fn count_floor(input: &str) -> i32 {
        count(input).0
    }

    fn first_base(input: &str) -> i32 {
        count(input).1
    }

    #[test]
    fn example() {
        assert_eq!(0, count_floor("(())"));
        assert_eq!(0, count_floor("()()"));
        assert_eq!(3, count_floor("((("));
        assert_eq!(3, count_floor("(()(()("));
        assert_eq!(3, count_floor("))((((("));
        assert_eq!(-1, count_floor("())"));
        assert_eq!(-1, count_floor("))("));
        assert_eq!(-3, count_floor(")))"));
        assert_eq!(-3, count_floor(")())())"));

        assert_eq!(1, first_base(")"));
        assert_eq!(5, first_base("()())"));
    }
}

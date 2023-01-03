pub fn solve(input: &str) {
    let instructions = parse(input);
    let (floor, index) = count(&instructions);
    println!("Part 1: {floor}");
    println!("Part 2: {index}");
}

enum Instruction {
    GoUp,
    GoDown,
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .chars()
        .map(|c| match c {
            '(' => Instruction::GoUp,
            ')' => Instruction::GoDown,
            _ => panic!("Unexpected char: {c}"),
        })
        .collect()
}

fn count(instructions: &[Instruction]) -> (i32, i32) {
    let mut floor = 0;
    let mut index = 1;
    let mut reached_basement = false;

    for instuction in instructions {
        floor += match instuction {
            Instruction::GoUp => 1,
            Instruction::GoDown => -1,
        };

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
    use super::*;

    fn count_floor(input: &str) -> i32 {
        let instructions = parse(input);
        count(&instructions).0
    }

    fn first_base(input: &str) -> i32 {
        let instructions = parse(input);
        count(&instructions).1
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

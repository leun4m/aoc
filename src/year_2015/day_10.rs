pub fn solve(input: &str) {
    let mut result_one = input.to_string();
    (0..40).for_each(|_| result_one = look_and_say(&result_one));
    let mut result_two = result_one.clone();
    println!("Part 1: {}", result_one.len());
    (0..10).for_each(|_| result_two = look_and_say(&result_two));
    println!("Part 2: {}", result_two.len());
}

const NULL_CHAR: char = '_';

fn look_and_say(input: &str) -> String {
    let mut result = String::new();
    let mut previous = NULL_CHAR;
    let mut times = 1;

    for char in input.chars() {
        if previous == NULL_CHAR {
            previous = char;
        } else if char == previous {
            times += 1;
        } else {
            result.push_str(&format!("{}{}", times, previous));
            previous = char;
            times = 1;
        }
    }
    result.push_str(&format!("{}{}", times, previous));
    result
}

#[cfg(test)]
mod test {
    use super::look_and_say;

    #[test]
    fn example() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}

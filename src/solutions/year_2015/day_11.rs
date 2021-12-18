pub fn solve(input: &str) {
    let mut new_password = increment(input);

    if !input.is_empty() {
        while !is_valid(&new_password) {
            new_password = increment(&new_password);
        }
    }

    println!("{}", new_password)
}

fn increment(input: &str) -> String {
    let mut result = String::new();
    let mut has_to_increment = true;

    for c in input.chars().rev() {
        let new_char = if has_to_increment {
            if c < 'z' {
                has_to_increment = false;
                (c as u8 + 1) as char
            } else {
                'a'
            }
        } else {
            c
        };
        result = format!("{}{}", new_char, result);
    }
    result
}

fn is_valid(input: &str) -> bool {
    if input.chars().count() < 3 {
        return false;
    }

    let mut contains_street = false;
    let mut pairs = Vec::new();

    let mut chars = input.chars();
    let mut pre_previous = chars.next().unwrap();
    let mut previous = chars.next().unwrap();

    for c in chars {
        if ['i', 'o', 'l'].contains(&c) {
            return false;
        }

        if c as u32 == previous as u32 + 1 && previous as u32 == pre_previous as u32 + 1 {
            contains_street = true;
        }

        if c == previous && previous != pre_previous && !pairs.contains(&c) {
            pairs.push(c);
        }

        pre_previous = previous;
        previous = c;
    }
    contains_street && pairs.len() >= 2
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        assert_eq!("abc", super::increment("abb"))
    }
}

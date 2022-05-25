pub fn solve(input: &str) {
    let decompressed = decompress(input);
    println!("Part 1: {}", decompressed.len())
}

fn sanatize(input: &str) -> String {
    input.replace(|c: char| c.is_ascii_whitespace(), "")
}

fn decompress(input: &str) -> String {
    let mut result = String::new();

    let s = sanatize(input);
    let mut chars = s.chars();
    while let Some(x) = chars.next() {
        match x {
            '(' => {
                let count = read_next_num(&mut chars);
                let times = read_next_num(&mut chars);

                result.push_str(&extract_chars(&mut chars, count).repeat(times));
            }
            c => result.push(c),
        };
    }

    result
}

fn extract_chars(chars: &mut std::str::Chars, count: usize) -> String {
    let mut result = String::new();
    
    for c in chars.take(count) {
        result.push(c);
    }

    result
}

fn read_next_num(chars: &mut std::str::Chars) -> usize {
    let mut num = String::new();
    let mut found_end = true;

    while found_end {
        if let Some(x) = chars.next() {
            if x.is_digit(10) {
                num.push(x);
            } else {
                found_end = false;
            }
        }
    }
    
    num.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_works() {
        assert_eq!(&decompress("ADVENT"), "ADVENT");
        assert_eq!(&decompress("A(1x5)BC"), "ABBBBBC");
        assert_eq!(&decompress("(3x3)XYZ"), "XYZXYZXYZ");
    }
}

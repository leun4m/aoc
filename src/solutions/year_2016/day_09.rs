pub fn solve(input: &str) {
    let sanatized_input = sanatize(input);
    println!("Part 1: {}", decompress_v1(&sanatized_input).len());
    println!("Part 2: {}", decompress_v2_len(&sanatized_input));
}

fn sanatize(input: &str) -> String {
    input.replace(|c: char| c.is_ascii_whitespace(), "")
}

fn decompress_v1(input: &str) -> String {
    let mut result = String::new();

    let mut chars = input.chars();
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

fn decompress_v2_len(input: &str) -> usize {
    parse(input).iter().map(|x| x.size()).sum()
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Repeat(Vec<Token>, usize),
    Char(char),
}

impl Token {
    fn size(&self) -> usize {
        match self {
            Token::Repeat(vec, size) => vec.iter().map(|x| x.size()).sum::<usize>() * size,
            Token::Char(_) => 1,
        }
    }
}

fn parse(input: &str) -> Vec<Token> {
    let mut result = Vec::new();
    let mut chars = input.chars();

    while let Some(x) = chars.next() {
        result.push(match x {
            '(' => {
                let count = read_next_num(&mut chars);
                let times = read_next_num(&mut chars);

                let next = extract_chars(&mut chars, count);
                Token::Repeat(parse(&next), times)
            }
            c => Token::Char(c),
        });
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
        assert_eq!(&decompress_v1("ADVENT"), "ADVENT");
        assert_eq!(&decompress_v1("A(1x5)BC"), "ABBBBBC");
        assert_eq!(&decompress_v1("(3x3)XYZ"), "XYZXYZXYZ");
    }
    
    #[test]
    fn decompress_v2_works() {
        assert_eq!(decompress_v2_len("(3x3)XYZ"), 9);
        assert_eq!(decompress_v2_len("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(decompress_v2_len("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    }
}

pub fn solve(input: &str) {
    let sanatized_input = sanatize(input);
    println!("Part 1: {}", part_one(&sanatized_input));
    println!("Part 2: {}", part_two(&sanatized_input));
}

fn calc_decompressed_len<F>(input: &str, parser: F) -> usize
where
    F: Fn(char, &mut std::str::Chars) -> Token,
{
    parse(input, parser).iter().map(Token::size).sum::<usize>()
}

fn part_one(input: &str) -> usize {
    calc_decompressed_len(input, parse_token_v1)
}

fn part_two(input: &str) -> usize {
    calc_decompressed_len(input, parse_token_v2)
}

fn sanatize(input: &str) -> String {
    input.replace(|c: char| c.is_ascii_whitespace(), "")
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    RepeatV1(String, usize),
    RepeatV2(Vec<Token>, usize),
    Char(char),
}

impl Token {
    fn size(&self) -> usize {
        match self {
            Token::RepeatV1(word, size) => word.len() * size,
            Token::RepeatV2(vec, size) => vec.iter().map(|x| x.size()).sum::<usize>() * size,
            Token::Char(_) => 1,
        }
    }
}

fn parse<F>(input: &str, parse_token: F) -> Vec<Token>
where
    F: Fn(char, &mut std::str::Chars) -> Token,
{
    let mut result = Vec::new();
    let mut chars = input.chars();

    while let Some(x) = chars.next() {
        result.push(parse_token(x, &mut chars));
    }

    result
}

fn parse_token_v1(x: char, chars: &mut std::str::Chars) -> Token {
    match x {
        '(' => {
            let count = read_next_num(chars);
            let times = read_next_num(chars);
            let next = chars.take(count).collect::<String>();

            Token::RepeatV1(next, times)
        }
        c => Token::Char(c),
    }
}

fn parse_token_v2(x: char, chars: &mut std::str::Chars) -> Token {
    match x {
        '(' => {
            let count = read_next_num(chars);
            let times = read_next_num(chars);
            let next = chars.take(count).collect::<String>();

            Token::RepeatV2(parse(&next, parse_token_v2), times)
        }
        c => Token::Char(c),
    }
}

fn read_next_num(chars: &mut std::str::Chars) -> usize {
    let mut num = String::new();

    while let Some(x) = chars.next() {
        if x.is_digit(10) {
            num.push(x);
        } else {
            break;
        }
    }

    num.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_works() {
        assert_eq!(part_one("ADVENT"), "ADVENT".len());
        assert_eq!(part_one("A(1x5)BC"), "ABBBBBC".len());
        assert_eq!(part_one("(3x3)XYZ"), "XYZXYZXYZ".len());
    }

    #[test]
    fn decompress_v2_works() {
        assert_eq!(part_two("(3x3)XYZ"), 9);
        assert_eq!(part_two("X(8x2)(3x3)ABCY"), 20);
        assert_eq!(part_two("(27x12)(20x12)(13x14)(7x10)(1x12)A"), 241920);
    }
}

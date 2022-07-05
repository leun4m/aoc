pub fn solve(input: &str) {
    let sanatized_input = sanatize(input);
    println!("Part 1: {}", part_one(&sanatized_input));
    println!("Part 2: {}", part_two(&sanatized_input));
}

fn calc_decompressed_len<F>(input: &str, parse_repeat: F) -> usize
where
    F: Fn(String, usize) -> Token,
{
    parse(input, parse_repeat)
        .iter()
        .map(Token::size)
        .sum::<usize>()
}

fn part_one(input: &str) -> usize {
    fn parser(following: String, times: usize) -> Token {
        Token::RepeatV1(following, times)
    }

    calc_decompressed_len(input, parser)
}

fn part_two(input: &str) -> usize {
    fn parser(following: String, times: usize) -> Token {
        Token::RepeatV2(parse(&following, parser), times)
    }

    calc_decompressed_len(input, parser)
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

fn parse<F>(input: &str, parse_repeat: F) -> Vec<Token>
where
    F: Fn(String, usize) -> Token,
{
    let mut result = Vec::new();
    let mut chars = input.chars();

    while let Some(x) = &mut chars.next() {
        result.push(match x {
            '(' => {
                let count = read_next_num(&mut chars);
                let times = read_next_num(&mut chars);
                let following = (&mut chars).take(count).collect();

                parse_repeat(following, times)
            }
            c => Token::Char(*c),
        });
    }

    result
}

fn read_next_num(chars: &mut std::str::Chars) -> usize {
    let mut num = String::new();

    for c in chars.by_ref() {
        if c.is_ascii_digit() {
            num.push(c);
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

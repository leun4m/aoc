use regex::Regex;

const REPLACEMENT_STR: &str = "_";

pub fn main(input: &str) {
    let result = count(input);
    let encoded = count(&encode(input));
    println!(
        "P1: {} ({} - {})",
        result.calc(),
        result.number_of_code,
        result.number_of_chars
    );
    println!(
        "P2: {} ({} - {})",
        encoded.calc(),
        encoded.number_of_code,
        encoded.number_of_chars
    );
}

fn encode(input: &str) -> String {
    let mut result = String::new();

    for line in input.lines() {
        let line_enc = line.replace("\\", "\\\\").replace("\"", "\\\"");
        result.push_str(&format!("\"{}\"\n", line_enc));
    }

    result
}

struct RiddleResult {
    number_of_code: u32,
    number_of_chars: u32,
}

impl RiddleResult {
    fn new(input: &str) -> Self {
        Self {
            number_of_code: count_chars(input),
            number_of_chars: 0,
        }
    }

    fn calc(&self) -> u32 {
        self.number_of_code - self.number_of_chars
    }
}

fn count(input: &str) -> RiddleResult {
    let mut result = RiddleResult::new(input);
    let regex = Regex::new(r#"^"(.*)"$"#).unwrap();
    let hex_char = Regex::new(r"\\x[0-9a-fA-F]{2}").unwrap();
    let mut counter = 0;
    for line in input.lines() {
        let mut inside = regex
            .captures(line)
            .expect(&format!("This is not surrounded by \": {}", line))
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .replace(r#"\""#, REPLACEMENT_STR)
            .replace(r"\\", REPLACEMENT_STR);
        inside = hex_char.replace_all(&inside, REPLACEMENT_STR).to_string();
        counter += count_chars(&inside);
    }
    result.number_of_chars = counter;
    result
}

fn count_chars(input: &str) -> u32 {
    input.replace("\n", "").chars().count() as u32
}

#[cfg(test)]
mod test {
    use super::{count, encode};

    #[test]
    fn example_quotes() {
        let input = r#""""#;
        let result = count(input);
        assert_eq!(2, result.number_of_code);
        assert_eq!(0, result.number_of_chars);

        let encoded = encode(input);
        let encoded_result = count(&encoded);
        assert_eq!(r#""\"\"""#, encoded.trim_end());
        assert_eq!(6, encoded_result.number_of_code);
        assert_eq!(2, encoded_result.number_of_chars);
    }

    #[test]
    fn example_abc() {
        let input = r#""abc""#;
        let result = count(input);
        assert_eq!(5, result.number_of_code);
        assert_eq!(3, result.number_of_chars);

        let encoded = encode(input);
        let encoded_result = count(&encoded);
        assert_eq!(r#""\"abc\"""#, encoded.trim_end());
        assert_eq!(9, encoded_result.number_of_code);
        assert_eq!(5, encoded_result.number_of_chars);
    }

    #[test]
    fn example_escaped_quote() {
        let input = r#""aaa\"aaa""#;
        let result = count(input);
        assert_eq!(10, result.number_of_code);
        assert_eq!(7, result.number_of_chars);

        let encoded = encode(input);
        let encoded_result = count(&encoded);
        assert_eq!(r#""\"aaa\\\"aaa\"""#, encoded.trim_end());
        assert_eq!(16, encoded_result.number_of_code);
        assert_eq!(10, encoded_result.number_of_chars);
    }

    #[test]
    fn example_hexadecimal() {
        let input = r#""\x27""#;
        let result = count(input);
        assert_eq!(6, result.number_of_code);
        assert_eq!(1, result.number_of_chars);

        let encoded = encode(input);
        let encoded_result = count(&encoded);
        assert_eq!(r#""\"\\x27\"""#, encoded.trim_end());
        assert_eq!(11, encoded_result.number_of_code);
        assert_eq!(6, encoded_result.number_of_chars);
    }

    #[test]
    fn example_all() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        let result = count(input);
        assert_eq!(23, result.number_of_code);
        assert_eq!(11, result.number_of_chars);
        assert_eq!(12, result.calc());

        let encoded = encode(input);
        let encoded_result = count(&encoded);
        assert_eq!(42, encoded_result.number_of_code);
        assert_eq!(23, encoded_result.number_of_chars);
        assert_eq!(19, encoded_result.calc());
    }

    #[test]
    fn custom() {
        let input = r#""l\xfampwtme\x69qvxnx\"\"\xc4jruuymjxrpsv""#;
        let result = count(input);
        assert_eq!(42, result.number_of_code);
        assert_eq!(29, result.number_of_chars);
        assert_eq!(
            r#""\"l\\xfampwtme\\x69qvxnx\\\"\\\"\\xc4jruuymjxrpsv\"""#,
            encode(input).trim_end()
        );
    }
}

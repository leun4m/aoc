use regex::Regex;
use std::collections::HashMap;

pub fn main(input: &str) {
    println!("Part 1: {}", part_one(input));
}

fn part_one(input: &str) -> i32 {
    let root = parse(input);
    sum_numbers(&root)
}

fn sum_numbers(root: &JSONElement) -> i32 {
    match root {
        JSONElement::JSONString(_) => 0,
        JSONElement::JSONNumber(x) => *x,
        JSONElement::JSONArray(v) => v.iter().map(|x| sum_numbers(x)).sum(),
        JSONElement::JSONObject(m) => m.values().map(|x| sum_numbers(x)).sum(),
    }
}

fn parse(input: &str) -> Box<JSONElement> {
    let trimmed = input.trim();
    let regex_number: Regex = Regex::new(r"^-?\d+$").unwrap();
    let regex_string: Regex = Regex::new(r#"^"([^"]*)"$"#).unwrap();
    if let Some(capture) = regex_number.captures(trimmed) {
        let value = capture[0].parse::<i32>().unwrap();
        return Box::new(JSONElement::JSONNumber(value));
    } else if let Some(capture) = regex_string.captures(trimmed) {
        let value = capture[1].parse::<String>().unwrap();
        return Box::new(JSONElement::JSONString(value));
    } else if trimmed.starts_with("[") && trimmed.ends_with("]") {
        if trimmed == "[]" {
            return Box::new(JSONElement::JSONArray(Vec::new()));
        } else {
            return parse_array(&trimmed[1..trimmed.len() - 1]);
        }
    } else if trimmed.starts_with("{") && trimmed.ends_with("}") {
        if trimmed == "{}" {
            return Box::new(JSONElement::JSONObject(HashMap::new()));
        } else {
            return parse_object(&input[1..input.len() - 1]);
        }
    }

    panic!("Unexpected Input: {}", input);
}

fn parse_array(inner: &str) -> Box<JSONElement> {
    let mut a: Vec<JSONElement> = Vec::new();

    let mut word = String::new();
    let mut count_brackets = 0;
    for c in inner.chars() {
        if count_brackets == 0 && c == ',' {
            a.push(*parse(&word));
            word = String::new();
        } else {
            word.push(c);
            match c {
                '[' | '{' => count_brackets += 1,
                ']' | '}' => count_brackets -= 1,
                _ => {}
            }
        }
    }

    a.push(*parse(&word));

    Box::new(JSONElement::JSONArray(a))
}

fn parse_object(inner: &str) -> Box<JSONElement> {
    let mut a: HashMap<String, JSONElement> = HashMap::new();

    let mut key = String::new();
    let mut word = String::new();
    let mut count_brackets = 0;
    let mut state = State::StartKey;

    for c in inner.chars() {
        if count_brackets == 0 && c == '"' && state.is_key() {
            state = match state {
                State::StartKey => State::EndKey,
                State::EndKey => State::StartValue,
                x => x,
            }
        } else if count_brackets == 0 && c == ':' {
            //
        } else if count_brackets == 0 && c == ',' {
            a.insert(key.clone(), *parse(&word));
            key = String::new();
            word = String::new();
            state = State::StartKey;
        } else {
            if c == '[' || c == '{' {
                count_brackets += 1;
            } else if c == ']' || c == '}' {
                count_brackets -= 1;
            }

            match state {
                State::StartKey => {
                    if !c.is_whitespace() {
                        key.push(c)
                    }
                }
                State::EndKey => key.push(c),
                State::StartValue => {
                    if !c.is_whitespace() {
                        word.push(c)
                    }
                }
            }
        }
    }
    a.insert(key, *parse(&word));

    Box::new(JSONElement::JSONObject(a))
}

enum State {
    StartKey,
    EndKey,
    StartValue,
    // EndValue,
}

impl State {
    fn is_key(&self) -> bool {
        match self {
            State::StartKey | State::EndKey => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum JSONElement {
    JSONObject(HashMap<String, JSONElement>),
    JSONArray(Vec<JSONElement>),
    JSONNumber(i32),
    JSONString(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_number_works() {
        assert_eq!(parse("1"), Box::new(JSONElement::JSONNumber(1)));
        assert_eq!(parse("-10"), Box::new(JSONElement::JSONNumber(-10)));
        assert_eq!(parse("12345"), Box::new(JSONElement::JSONNumber(12345)));
    }

    #[test]
    fn parse_string_works() {
        assert_eq!(
            parse("\"h\""),
            Box::new(JSONElement::JSONString("h".into()))
        );
        assert_eq!(
            parse("\"Test\""),
            Box::new(JSONElement::JSONString("Test".into()))
        );
        assert_eq!(parse("\"\""), Box::new(JSONElement::JSONString("".into())));
    }

    #[test]
    fn parse_array_works() {
        assert_eq!(parse("[]"), Box::new(JSONElement::JSONArray(Vec::new())));
        assert_eq!(
            parse("[\"a\"]"),
            Box::new(JSONElement::JSONArray(vec![JSONElement::JSONString(
                "a".into()
            )]))
        );

        assert_eq!(
            parse("[1,2,3]"),
            Box::new(JSONElement::JSONArray(vec![
                JSONElement::JSONNumber(1),
                JSONElement::JSONNumber(2),
                JSONElement::JSONNumber(3),
            ]))
        );
    }

    #[test]
    fn parse_object_works() {
        assert_eq!(
            parse("{}"),
            Box::new(JSONElement::JSONObject(HashMap::new()))
        );
        assert_eq!(
            parse("{\"a\":\"abc\"}"),
            Box::new(JSONElement::JSONObject(HashMap::from([(
                "a".into(),
                JSONElement::JSONString("abc".into())
            )])))
        );
        assert_eq!(
            parse("{\"a\":\"abc\", \"b\":123}"),
            Box::new(JSONElement::JSONObject(HashMap::from([
                ("a".into(), JSONElement::JSONString("abc".into())),
                ("b".into(), JSONElement::JSONNumber(123))
            ])))
        );
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(r#"[1,2,3]"#), 6);
        assert_eq!(part_one(r#"{"a":2,"b":4}"#), 6);

        assert_eq!(part_one(r#"[[[3]]]"#), 3);
        assert_eq!(part_one(r#"{"a":{"b":4},"c":-1}"#), 3);

        assert_eq!(part_one(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(part_one(r#"[-1,{"a":1}]"#), 0);

        assert_eq!(part_one(r#"[]"#), 0);
        assert_eq!(part_one(r#"{}"#), 0);
    }
}

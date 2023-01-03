use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: &str) {
    let root = parse(input);
    println!("Part 1: {}", part_one(&root));
    println!("Part 2: {}", part_two(&root));
}

fn part_one(root: &JSONElement) -> i32 {
    match root {
        JSONElement::Text(_) => 0,
        JSONElement::Number(x) => *x,
        JSONElement::Array(v) => v.iter().map(part_one).sum(),
        JSONElement::Object(m) => m.values().map(part_one).sum(),
    }
}

fn part_two(root: &JSONElement) -> i32 {
    let red_value: JSONElement = JSONElement::Text("red".into());
    match root {
        JSONElement::Text(_) => 0,
        JSONElement::Number(x) => *x,
        JSONElement::Array(v) => v.iter().map(part_two).sum(),
        JSONElement::Object(m) => {
            if m.values().any(|x| x == &red_value) {
                0
            } else {
                m.values().map(part_two).sum()
            }
        }
    }
}

fn parse(input: &str) -> JSONElement {
    let trimmed = input.trim();

    let regex_number: Regex = Regex::new(r#"^-?\d+$"#).unwrap();
    let regex_string: Regex = Regex::new(r#"^"([^"]*)"$"#).unwrap();

    if let Some(capture) = regex_number.captures(trimmed) {
        let value = capture[0].parse::<i32>().unwrap();
        JSONElement::Number(value)
    } else if let Some(capture) = regex_string.captures(trimmed) {
        let value = capture[1].parse::<String>().unwrap();
        JSONElement::Text(value)
    } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
        let value = parse_array(&trimmed[1..trimmed.len() - 1]);
        JSONElement::Array(value)
    } else if trimmed.starts_with('{') && trimmed.ends_with('}') {
        let value = parse_object(&trimmed[1..trimmed.len() - 1]);
        JSONElement::Object(value)
    } else {
        panic!("Unexpected Input: {trimmed}");
    }
}

fn parse_array(inner: &str) -> Vec<JSONElement> {
    let mut result: Vec<JSONElement> = Vec::new();
    let trimmed = inner.trim();

    if trimmed.is_empty() {
        return result;
    }

    let mut word = String::new();
    let mut count_brackets = 0;

    for c in trimmed.chars() {
        if count_brackets == 0 && c == ',' {
            result.push(parse(&word));
            word.clear();
        } else {
            word.push(c);
            count_brackets += get_mod_brackets(c);
        }
    }

    result.push(parse(&word));

    result
}

fn parse_object(inner: &str) -> HashMap<String, JSONElement> {
    let mut result: HashMap<String, JSONElement> = HashMap::new();
    let trimmed = inner.trim();

    if trimmed.is_empty() {
        return result;
    }

    let mut key = String::new();
    let mut value = String::new();
    let mut count_brackets = 0;
    let mut state = State::StartKey;

    for c in trimmed.chars() {
        state = if count_brackets == 0 && c == '"' && state.is_key() {
            match state {
                State::StartKey => State::EndKey,
                State::EndKey => State::StartValue,
                State::StartValue => state,
            }
        } else if count_brackets == 0 && c == ',' {
            result.insert(key.clone(), parse(&value));

            key.clear();
            value.clear();
            State::StartKey
        } else {
            count_brackets += get_mod_brackets(c);

            match state {
                State::StartKey => {
                    if !c.is_whitespace() {
                        key.push(c);
                    }
                }
                State::EndKey => key.push(c),
                State::StartValue => {
                    if !c.is_whitespace() && c != ':' {
                        value.push(c);
                    }
                }
            }
            state
        }
    }

    result.insert(key, parse(&value));

    result
}

fn get_mod_brackets(c: char) -> i32 {
    match c {
        '[' | '{' => 1,
        ']' | '}' => -1,
        _ => 0,
    }
}

enum State {
    StartKey,
    EndKey,
    StartValue,
    // EndValue,
}

impl State {
    fn is_key(&self) -> bool {
        matches!(self, State::StartKey | State::EndKey)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum JSONElement {
    Object(HashMap<String, JSONElement>),
    Array(Vec<JSONElement>),
    Number(i32),
    Text(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_works() {
        assert_eq!(parse("1"), JSONElement::Number(1));
        assert_eq!(parse("-10"), JSONElement::Number(-10));
        assert_eq!(parse("12345"), JSONElement::Number(12345));
    }

    #[test]
    fn parse_string_works() {
        assert_eq!(parse("\"h\""), JSONElement::Text("h".into()));
        assert_eq!(parse("\"Test\""), JSONElement::Text("Test".into()));
        assert_eq!(parse("\"\""), JSONElement::Text(String::new()));
    }

    #[test]
    fn parse_array_works() {
        assert_eq!(parse("[]"), JSONElement::Array(Vec::new()));
        assert_eq!(
            parse("[\"a\"]"),
            JSONElement::Array(vec![JSONElement::Text("a".into())])
        );

        assert_eq!(
            parse("[1,2,3]"),
            JSONElement::Array(vec![
                JSONElement::Number(1),
                JSONElement::Number(2),
                JSONElement::Number(3),
            ])
        );
    }

    #[test]
    fn parse_object_works() {
        assert_eq!(parse("{}"), JSONElement::Object(HashMap::new()));
        assert_eq!(
            parse("{\"a\":\"abc\"}"),
            JSONElement::Object(HashMap::from([(
                "a".into(),
                JSONElement::Text("abc".into())
            )]))
        );
        assert_eq!(
            parse("{\"a\":\"abc\", \"b\":123}"),
            JSONElement::Object(HashMap::from([
                ("a".into(), JSONElement::Text("abc".into())),
                ("b".into(), JSONElement::Number(123))
            ]))
        );
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&parse(r#"[1,2,3]"#)), 6);
        assert_eq!(part_one(&parse(r#"{"a":2,"b":4}"#)), 6);

        assert_eq!(part_one(&parse(r#"[[[3]]]"#)), 3);
        assert_eq!(part_one(&parse(r#"{"a":{"b":4},"c":-1}"#)), 3);

        assert_eq!(part_one(&parse(r#"{"a":[-1,1]}"#)), 0);
        assert_eq!(part_one(&parse(r#"[-1,{"a":1}]"#)), 0);

        assert_eq!(part_one(&parse(r#"[]"#)), 0);
        assert_eq!(part_one(&parse(r#"{}"#)), 0);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(&parse(r#"[1,2,3]"#)), 6);
        assert_eq!(part_two(&parse(r#"[1,{"c":"red","b":2},3]"#)), 4);
        assert_eq!(part_two(&parse(r#"{"d":"red","e":[1,2,3,4],"f":5}"#)), 0);
        assert_eq!(part_two(&parse(r#"[1,"red",5]"#)), 6);
    }
}

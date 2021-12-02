use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: &str) {
    let root = parse(input);
    println!("Part 1: {}", part_one(&root));
    println!("Part 2: {}", part_two(&root));
}

fn part_one(root: &JSONElement) -> i32 {
    match root {
        JSONElement::JSONString(_) => 0,
        JSONElement::JSONNumber(x) => *x,
        JSONElement::JSONArray(v) => v.iter().map(|x| part_one(x)).sum(),
        JSONElement::JSONObject(m) => m.values().map(|x| part_one(x)).sum(),
    }
}

fn part_two(root: &JSONElement) -> i32 {
    let red_value: JSONElement = JSONElement::JSONString("red".into());
    match root {
        JSONElement::JSONString(_) => 0,
        JSONElement::JSONNumber(x) => *x,
        JSONElement::JSONArray(v) => v.iter().map(|x| part_two(x)).sum(),
        JSONElement::JSONObject(m) => {
            if m.values().any(|x| x == &red_value) {
                0
            } else {
                m.values().map(|x| part_two(x)).sum()
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
        JSONElement::JSONNumber(value)
    } else if let Some(capture) = regex_string.captures(trimmed) {
        let value = capture[1].parse::<String>().unwrap();
        JSONElement::JSONString(value)
    } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
        let value = parse_array(&trimmed[1..trimmed.len() - 1]);
        JSONElement::JSONArray(value)
    } else if trimmed.starts_with('{') && trimmed.ends_with('}') {
        let value = parse_object(&trimmed[1..trimmed.len() - 1]);
        JSONElement::JSONObject(value)
    } else {
        panic!("Unexpected Input: {}", trimmed);
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
                x => x,
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
                        key.push(c)
                    }
                }
                State::EndKey => key.push(c),
                State::StartValue => {
                    if !c.is_whitespace() && c != ':' {
                        value.push(c)
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
    JSONObject(HashMap<String, JSONElement>),
    JSONArray(Vec<JSONElement>),
    JSONNumber(i32),
    JSONString(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_works() {
        assert_eq!(parse("1"), JSONElement::JSONNumber(1));
        assert_eq!(parse("-10"), JSONElement::JSONNumber(-10));
        assert_eq!(parse("12345"), JSONElement::JSONNumber(12345));
    }

    #[test]
    fn parse_string_works() {
        assert_eq!(parse("\"h\""), JSONElement::JSONString("h".into()));
        assert_eq!(parse("\"Test\""), JSONElement::JSONString("Test".into()));
        assert_eq!(parse("\"\""), JSONElement::JSONString("".into()));
    }

    #[test]
    fn parse_array_works() {
        assert_eq!(parse("[]"), JSONElement::JSONArray(Vec::new()));
        assert_eq!(
            parse("[\"a\"]"),
            JSONElement::JSONArray(vec![JSONElement::JSONString("a".into())])
        );

        assert_eq!(
            parse("[1,2,3]"),
            JSONElement::JSONArray(vec![
                JSONElement::JSONNumber(1),
                JSONElement::JSONNumber(2),
                JSONElement::JSONNumber(3),
            ])
        );
    }

    #[test]
    fn parse_object_works() {
        assert_eq!(parse("{}"), JSONElement::JSONObject(HashMap::new()));
        assert_eq!(
            parse("{\"a\":\"abc\"}"),
            JSONElement::JSONObject(HashMap::from([(
                "a".into(),
                JSONElement::JSONString("abc".into())
            )]))
        );
        assert_eq!(
            parse("{\"a\":\"abc\", \"b\":123}"),
            JSONElement::JSONObject(HashMap::from([
                ("a".into(), JSONElement::JSONString("abc".into())),
                ("b".into(), JSONElement::JSONNumber(123))
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

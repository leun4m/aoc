use regex::Regex;
use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, Shl, Shr};

type BaseType = u16;

const REGEX_INSTRUCTION: &str = r"^(.+) -> (\w+)$";
const REGEX_NUMBER: &str = r"^\d+$";
const REGEX_NAME: &str = r"^\w+$";
const REGEX_BINARY_OPERATOR: &str = r"^(\w+) (\w+) (\w+)";
const REGEX_NOT: &str = r"^NOT (\w+)";

const WIRE_TO_OBSERVE: &str = "a";
const WIRE_TO_CHANGE: &str = "b";

pub fn main(input: &str) {
    let result = internal(input, WIRE_TO_OBSERVE);
    println!("P1 - Wire a: {}", result.0);
    println!("P2 - Wire a: {}", result.1);
}

fn internal(input: &str, observed_wire: &str) -> (BaseType, BaseType) {
    let mut wires: HashMap<String, BaseType> = HashMap::new();
    let mut instructions = Vec::new();

    first_read(input, &mut wires, &mut instructions);
    apply_instructions(&mut wires, &instructions);
    let signal_one = *wires.get(observed_wire).unwrap();

    wires.clear();
    instructions.clear();

    first_read(input, &mut wires, &mut instructions);
    wires.insert(String::from(WIRE_TO_CHANGE), signal_one);
    apply_instructions(&mut wires, &instructions);
    let signal_two = *wires.get(observed_wire).unwrap();

    (signal_one, signal_two)
}

#[derive(Debug, PartialEq)]
enum Symbol {
    NUMBER(BaseType),
    NAME(String),
}

impl Symbol {
    fn parse(input: &str) -> Option<Self> {
        let reg_number = Regex::new(REGEX_NUMBER).unwrap();
        let cap_num = reg_number.captures(input);

        if input.contains(' ') {
            Option::None
        } else if cap_num.is_some() {
            let signal = cap_num
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .parse::<BaseType>()
                .unwrap();
            Some(Symbol::NUMBER(signal))
        } else {
            Some(Symbol::NAME(input.to_string()))
        }
    }

    fn interpret(&self, wires: &HashMap<String, BaseType>) -> Option<BaseType> {
        match self {
            Symbol::NUMBER(v) => Some(*v),
            Symbol::NAME(v) => {
                let val = wires.get(v);
                if let Some(x) = val {
                    Some(*x)
                } else {
                    None
                }
            }
        }
    }
}

/// The last value always represents the value to assign to
#[derive(Debug, PartialEq)]
enum Instruction {
    ASSIGN(Symbol, String),
    NOT(Symbol, String),
    AND(Symbol, Symbol, String),
    OR(Symbol, Symbol, String),
    LSHIFT(Symbol, Symbol, String),
    RSHIFT(Symbol, Symbol, String),
}

impl Instruction {
    fn get_aim(&self) -> String {
        match self {
            Instruction::ASSIGN(_, z)
            | Instruction::NOT(_, z)
            | Instruction::AND(_, _, z)
            | Instruction::OR(_, _, z)
            | Instruction::LSHIFT(_, _, z)
            | Instruction::RSHIFT(_, _, z) => z.clone(),
        }
    }

    fn from_binary(left: &str, operator: &str, right: &str, wire: String) -> Self {
        let sym_left = Symbol::parse(left).unwrap();
        let sym_right = Symbol::parse(right).unwrap();
        match operator {
            "AND" => Instruction::AND(sym_left, sym_right, wire),
            "OR" => Instruction::OR(sym_left, sym_right, wire),
            "LSHIFT" => Instruction::LSHIFT(sym_left, sym_right, wire),
            "RSHIFT" => Instruction::RSHIFT(sym_left, sym_right, wire),
            _ => {
                panic!("Unknown Operator: {}", operator);
            }
        }
    }

    fn parse(
        reg_name: &Regex,
        reg_binary_operator: &Regex,
        reg_not: &Regex,
        expression: &str,
        wire: &str,
        line: &str,
    ) -> Self {
        let cap_binary = reg_binary_operator.captures(expression);
        let cap_name = reg_name.captures(expression);
        let cap_not = reg_not.captures(expression);

        if cap_name.is_some() {
            Instruction::ASSIGN(Symbol::parse(expression).unwrap(), wire.to_string())
        } else if let Some(not) = cap_not {
            Instruction::NOT(
                Symbol::parse(not.get(1).unwrap().as_str()).unwrap(),
                wire.to_string(),
            )
        } else if let Some(op_expression) = cap_binary {
            Instruction::from_binary(
                op_expression.get(1).unwrap().as_str(),
                op_expression.get(2).unwrap().as_str(),
                op_expression.get(3).unwrap().as_str(),
                wire.to_string(),
            )
        } else {
            panic!("Line looks weird: {}", line);
        }
    }
}

fn apply_instructions(wires: &mut HashMap<String, BaseType>, instructions: &[Instruction]) {
    let mut applied_instructions = Vec::new();
    while applied_instructions.len() < instructions.len() {
        for instruction in instructions {
            let signal = perform_operation(&wires, &instruction);
            if let Some(sig) = signal {
                if !applied_instructions.contains(&instruction) {
                    wires.insert(instruction.get_aim(), sig);
                    applied_instructions.push(instruction);
                }
            }
        }
    }
}

fn first_read(
    input: &str,
    wires: &mut HashMap<String, BaseType>,
    instructions: &mut Vec<Instruction>,
) {
    let reg_instruction = Regex::new(REGEX_INSTRUCTION).unwrap();
    let reg_binary_operator = Regex::new(REGEX_BINARY_OPERATOR).unwrap();
    let reg_name = Regex::new(REGEX_NAME).unwrap();
    let reg_not = Regex::new(REGEX_NOT).unwrap();

    for line in input.lines() {
        let captures = reg_instruction
            .captures(line)
            .unwrap_or_else(|| panic!("Invalid line: {}", &line));

        let wire = captures.get(2).unwrap().as_str().to_string();
        let expression = captures.get(1).unwrap().as_str();
        let option_symbol = Symbol::parse(expression);
        if let Some(symbol) = option_symbol {
            if let Symbol::NUMBER(a) = &symbol {
                wires.insert(wire, *a);
            } else {
                instructions.push(Instruction::ASSIGN(symbol, wire))
            }
        } else {
            instructions.push(Instruction::parse(
                &reg_name,
                &reg_binary_operator,
                &reg_not,
                expression,
                &wire,
                line,
            ))
        }
    }
}

fn perform_operation(
    wires: &HashMap<String, BaseType>,
    instruction: &Instruction,
) -> Option<BaseType> {
    match &instruction {
        Instruction::ASSIGN(a, _) => a.interpret(&wires),
        Instruction::NOT(a, _) => {
            let option = a.interpret(&wires);
            if let Some(x) = option {
                Some(!x)
            } else {
                None
            }
        }
        Instruction::AND(a, b, _) => {
            let option_a = a.interpret(&wires);
            let option_b = b.interpret(&wires);
            apply_binary(option_a, option_b, BaseType::bitand)
        }
        Instruction::OR(a, b, _) => {
            let option_a = a.interpret(&wires);
            let option_b = b.interpret(&wires);
            apply_binary(option_a, option_b, BaseType::bitor)
        }
        Instruction::LSHIFT(a, b, _) => {
            let option_a = a.interpret(&wires);
            let option_b = b.interpret(&wires);
            apply_binary(option_a, option_b, BaseType::shl)
        }
        Instruction::RSHIFT(a, b, _) => {
            let option_a = a.interpret(&wires);
            let option_b = b.interpret(&wires);
            apply_binary(option_a, option_b, BaseType::shr)
        }
    }
}

fn apply_binary<F>(a: Option<BaseType>, b: Option<BaseType>, f: F) -> Option<BaseType>
where
    F: Fn(BaseType, BaseType) -> BaseType,
{
    if let Some(x) = a {
        if let Some(y) = b {
            return Some(f(x, y));
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::internal;

    #[test]
    fn example() {
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        assert_eq!(internal(input, "d").0, 72);
        assert_eq!(internal(input, "e").0, 507);
        assert_eq!(internal(input, "f").0, 492);
        assert_eq!(internal(input, "g").0, 114);
        assert_eq!(internal(input, "h").0, 65412);
        assert_eq!(internal(input, "i").0, 65079);
        assert_eq!(internal(input, "x").0, 123);
        assert_eq!(internal(input, "y").0, 456);
    }
}

use std::cmp::max;
use std::cmp::min;

pub fn solve(input: &str) {
    let packet = parse(input);
    println!("Part 1: {}", part_one(&packet));
    println!("Part 2: {}", part_two(&packet));
}

fn decode_4bit(input: char) -> String {
    format!("{:04b}", input.to_digit(16).unwrap())
}

fn bin_to_u8(input: &str) -> u8 {
    if input.is_empty() {
        0
    } else {
        u8::from_str_radix(input, 2).unwrap()
    }
}

const VERSION_LENGTH: usize = 3;
const ID_LENGTH: usize = 3;

fn parse(input: &str) -> Packet {
    let mut bin_iter = BinaryIter::new(input);
    parse_continue(&mut bin_iter).unwrap()
}

fn bit(chars: &[char]) -> String {
    let bits = format!("{}0000", &chars.iter().collect::<String>());
    format!("{:01x}", bin_to_u8(&bits[..4]))
}

fn bitstream(chars: &[char]) -> String {
    chars.chunks(4).map(|bits| bit(bits)).collect::<String>()
}

fn parse_bin(binary: &mut BinaryIter, length: usize) -> Vec<Packet> {
    let substr = binary.take(length).collect::<Vec<char>>();
    let sub_hex = bitstream(&substr);

    let mut sub_iter = BinaryIter::new(&sub_hex);
    parse_list(&mut sub_iter)
}

fn parse_continue(bin_iter: &mut BinaryIter) -> Option<Packet> {
    // log::trace!("iter #1: {:?}", bin_iter);
    let version = bin_to_u8(&bin_iter.take(VERSION_LENGTH).collect::<String>());
    // log::trace!("iter #2: {:?}", bin_iter);
    let id = bin_to_u8(&bin_iter.take(ID_LENGTH).collect::<String>());
    // log::trace!("iter #3: {:?}", bin_iter);

    if bin_iter.has_next() {
        let packet = match id {
            4 => Packet::Literal(LiteralPacket::new(version, id, bin_iter)),
            _ => Packet::Operator(OperatorPacket::new(version, id, bin_iter)),
        };
        Some(packet)
    } else {
        None
    }
}

fn parse_list(bin_iter: &mut BinaryIter) -> Vec<Packet> {
    let mut result = Vec::new();
    // log::trace!("iter: {:?} / {}", bin_iter, bin_iter.has_next());
    while bin_iter.has_next() {
        // log::trace!("parse_list: {:?}", result);
        if let Some(p) = parse_continue(bin_iter) {
            result.push(p);
        }
    }
    result
}

#[derive(Debug)]
struct BinaryIter<'a> {
    hex_str: &'a str,
    current_idx: usize,
    current_bin: String,
    ptr_bin: usize,
}

impl<'a> BinaryIter<'a> {
    fn new(hex_str: &'a str) -> Self {
        let current_bin = if let Some(first_char) = hex_str.chars().nth(0) {
            decode_4bit(first_char)
        } else {
            String::new()
        };

        Self {
            hex_str,
            current_idx: 0,
            current_bin,
            ptr_bin: 0,
        }
    }

    fn chunks(&mut self, size: usize) -> Vec<char> {
        let mut result = Vec::new();
        for _ in 0..size {
            if let Some(c) = self.next() {
                result.push(c);
            }
        }
        result
    }

    fn has_next(&self) -> bool {
        self.ptr_bin + 1 < self.current_bin.len() || self.current_idx + 1 < self.hex_str.len()
    }
}

impl<'a> Iterator for BinaryIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr_bin < 4 {
            self.ptr_bin += 1;
            self.current_bin.chars().nth(self.ptr_bin - 1)
        } else {
            self.current_idx += 1;
            if let Some(c) = self.hex_str.chars().nth(self.current_idx) {
                self.current_bin = decode_4bit(c);
                self.ptr_bin = 0;
                self.next()
            } else {
                None
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

impl Packet {
    fn sum_version(&self) -> u64 {
        match self {
            Packet::Literal(l) => l.version as u64,
            Packet::Operator(o) => {
                let own: u64 = o.version as u64;
                let sub: u64 = o.subpackets.iter().map(|p| p.sum_version()).sum();
                own + sub
            }
        }
    }

    fn value(&self) -> u64 {
        match self {
            Packet::Literal(packet) => packet.value as u64,
            Packet::Operator(packet) => packet.value(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LiteralPacket {
    version: u8,
    id: u8,
    value: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct OperatorPacket {
    version: u8,
    id: u8,
    subpackets: Vec<Packet>,
}

impl LiteralPacket {
    fn new(version: u8, id: u8, binary: &mut BinaryIter) -> Self {
        let mut repeat = true;
        let mut value_str = String::new();
        while repeat {
            let mut first = true;
            let a = binary.chunks(5);
            for c in a {
                if first {
                    if c == '0' {
                        repeat = false;
                    }
                    first = false
                } else {
                    value_str.push(c);
                }
            }
        }

        let value = u64::from_str_radix(&value_str, 2).unwrap();

        Self { version, id, value }
    }
}

const LENGTH_SUB_PACKETS: usize = 15;
const NUMBER_SUB_PACKETS: usize = 11;

impl OperatorPacket {
    fn subpackets_length(binary: &mut BinaryIter) -> Vec<Packet> {
        let s = binary.chunks(LENGTH_SUB_PACKETS).iter().collect::<String>();
        let length = usize::from_str_radix(&s, 2).unwrap();

        // log::trace!("S: {}", s);
        // log::trace!("LENGTH: {}", length);

        parse_bin(binary, length)
    }

    fn subpackets_dyn(binary: &mut BinaryIter) -> Vec<Packet> {
        let num_packets = usize::from_str_radix(
            &binary.chunks(NUMBER_SUB_PACKETS).iter().collect::<String>(),
            2,
        )
        .unwrap();

        // log::trace!("NUM_PACKETS: {}", num_packets);

        let mut subpackets = Vec::new();
        for _ in 0..num_packets {
            if let Some(p) = parse_continue(binary) {
                subpackets.push(p);
            }
        }

        subpackets
    }

    fn new(version: u8, id: u8, binary: &mut BinaryIter) -> Self {
        log::trace!("Operator iter #1: {:?}", binary);
        // log::trace!("Operator: {}", binary.hex_str);
        let subpackets = match binary.next() {
            Some('0') => OperatorPacket::subpackets_length(binary),
            Some('1') => OperatorPacket::subpackets_dyn(binary),
            Some(x) => panic!("Unexpected digit: {}", x),
            None => Vec::new(),
        };

        Self {
            version,
            id,
            subpackets,
        }
    }

    fn value(&self) -> u64 {
        let f = match self.id {
            0 => |a: u64, b: u64| a + b,
            1 => |a: u64, b: u64| a * b,
            2 => |a: u64, b: u64| min(a, b),
            3 => |a: u64, b: u64| max(a, b),
            5 => |a: u64, b: u64| if a > b { 1 } else { 0 },
            6 => |a: u64, b: u64| if a < b { 1 } else { 0 },
            7 => |a: u64, b: u64| if a == b { 1 } else { 0 },
            x => panic!("Unexpected id: {}", x),
        };

        println!("subpackets: {:?}", self.subpackets);

        self.subpackets
            .iter()
            .map(|a| a.value())
            .reduce(|a, b| f(a, b))
            .unwrap_or(0)
    }
}

fn part_one(packet: &Packet) -> u64 {
    packet.sum_version()
}

fn part_two(packet: &Packet) -> u64 {
    packet.value()
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn parse_literal_works() {
        assert_eq!(
            parse("D2FE28"),
            Packet::Literal(LiteralPacket {
                version: 6,
                id: 4,
                value: 2021
            })
        );
    }

    #[test]
    fn parse_operator_length_works() {
        assert_eq!(
            parse("38006F45291200"),
            Packet::Operator(OperatorPacket {
                version: 1,
                id: 6,
                subpackets: vec![
                    Packet::Literal(LiteralPacket {
                        version: 6,
                        id: 4,
                        value: 10,
                    }),
                    Packet::Literal(LiteralPacket {
                        version: 2,
                        id: 4,
                        value: 20,
                    })
                ]
            })
        )
    }

    #[test]
    fn bit_works() {
        assert_eq!(bit(&['1', '0', '1', '0']), String::from("a"));
    }

    #[test]
    fn bitstream_works() {
        assert_eq!(
            bitstream(&[
                '1', '1', '0', '1', '0', '0', '0', '1', '0', '1', '0', '0', '1', '0', '1', '0',
                '0', '1', '0', '0', '0', '1', '0', '0', '1', '0', '0'
            ]),
            String::from("d14a448")
        );
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(&parse("8A004A801A8002F478")), 16);
        assert_eq!(part_one(&parse("620080001611562C8802118E34")), 12);
        assert_eq!(part_one(&parse("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part_one(&parse("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(&parse("C200B40A82")), 3);
        println!("=========");
        assert_eq!(part_two(&parse("04005AC33890")), 54);
        assert_eq!(part_two(&parse("880086C3E88112")), 7);
        assert_eq!(part_two(&parse("CE00C43D881120")), 9);
        assert_eq!(part_two(&parse("D8005AC2A8F0")), 1);
        assert_eq!(part_two(&parse("F600BC2D8F")), 0);
        assert_eq!(part_two(&parse("9C005AC2F8F0")), 0);
        assert_eq!(part_two(&parse("9C0141080250320F1802104A08")), 1);
    }
}

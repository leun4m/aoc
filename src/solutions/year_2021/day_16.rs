use std::cmp::max;
use std::cmp::min;

pub fn solve(input: &str) {
    let packet = parse(input);
    println!("Part 1: {}", part_one(&packet));
    println!("Part 2: {}", part_two(&packet));
}

fn parse(input: &str) -> Packet {
    let mut bin_iter = BinaryIter::new(input);
    try_parse(&mut bin_iter).unwrap()
}

fn part_one(packet: &Packet) -> u64 {
    packet.sum_version()
}

fn part_two(packet: &Packet) -> u64 {
    packet.value()
}

fn decode_hex_to_nibble(hex: char) -> String {
    format!("{:04b}", hex.to_digit(16).unwrap())
}

fn decode_binary_to_hex(bits: &[char]) -> String {
    // to ensure zeros at the ending
    let bits = format!("{}0000", &bits.iter().collect::<String>());
    format!("{:01x}", bin_to_num(&bits[..4]))
}

fn decode_nibbles_to_hex(bits: &[char]) -> String {
    bits.chunks(4).map(decode_binary_to_hex).collect::<String>()
}

fn bin_to_num(input: &str) -> u64 {
    if input.is_empty() {
        0
    } else {
        u64::from_str_radix(input, 2).unwrap()
    }
}

const VERSION_LENGTH: usize = 3;
const ID_LENGTH: usize = 3;

fn try_parse(iter: &mut BinaryIter) -> Option<Packet> {
    let version = iter.take_and_convert(VERSION_LENGTH) as u8;
    let id = iter.take_and_convert(ID_LENGTH) as u8;

    if iter.has_next() {
        let packet = match id {
            4 => Packet::Literal(LiteralPacket::new(version, id, iter)),
            _ => Packet::Operator(OperatorPacket::new(version, id, iter)),
        };
        Some(packet)
    } else {
        None
    }
}

fn parse_packets_by_length(iter: &mut BinaryIter, length: usize) -> Vec<Packet> {
    let sub_bin = iter.take(length).collect::<Vec<char>>();
    let sub_hex = decode_nibbles_to_hex(&sub_bin);

    parse_packets(&mut BinaryIter::new(&sub_hex))
}

fn parse_packets(iter: &mut BinaryIter) -> Vec<Packet> {
    let mut result = Vec::new();
    while let Some(p) = try_parse(iter) {
        result.push(p);
    }
    result
}

#[derive(Debug)]
struct BinaryIter<'a> {
    /// The HEX string as a base
    hex: &'a str,
    /// The index of the current HEX-Chars
    hex_idx: usize,
    /// The current HEX-char as binary representation
    bin: String,
    /// The index of the current binary char
    bin_idx: usize,
}

impl<'a> BinaryIter<'a> {
    fn new(hex: &'a str) -> Self {
        let bin = if let Some(first_char) = hex.chars().next() {
            decode_hex_to_nibble(first_char)
        } else {
            String::new()
        };

        Self {
            hex,
            hex_idx: 0,
            bin,
            bin_idx: 0,
        }
    }

    fn chunks(&mut self, length: usize) -> Vec<char> {
        let mut result = Vec::new();
        for _ in 0..length {
            if let Some(c) = self.next() {
                result.push(c);
            }
        }
        result
    }

    fn take_and_convert(&mut self, length: usize) -> u64 {
        bin_to_num(&self.take(length).collect::<String>())
    }

    fn has_next(&self) -> bool {
        self.bin_idx + 1 < self.bin.len() || self.hex_idx + 1 < self.hex.len()
    }
}

impl<'a> Iterator for BinaryIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bin_idx < 4 {
            self.bin_idx += 1;
            self.bin.chars().nth(self.bin_idx - 1)
        } else {
            self.hex_idx += 1;
            if let Some(c) = self.hex.chars().nth(self.hex_idx) {
                self.bin = decode_hex_to_nibble(c);
                self.bin_idx = 0;
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
            Packet::Literal(l) => u64::from(l.version),
            Packet::Operator(o) => {
                let own: u64 = u64::from(o.version);
                let sub: u64 = o.subpackets.iter().map(Packet::sum_version).sum();
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
        let mut digits = String::new();

        while repeat {
            let mut first = true;
            for digit in binary.chunks(5) {
                if first {
                    if digit == '0' {
                        repeat = false;
                    }
                    first = false;
                } else {
                    digits.push(digit);
                }
            }
        }

        let value = bin_to_num(&digits);

        Self { version, id, value }
    }
}

const LENGTH_SUB_PACKETS: usize = 15;
const NUMBER_SUB_PACKETS: usize = 11;

impl OperatorPacket {
    fn subpackets_by_length(binary: &mut BinaryIter) -> Vec<Packet> {
        let length = binary.take_and_convert(LENGTH_SUB_PACKETS) as usize;

        parse_packets_by_length(binary, length)
    }

    fn subpackets_by_number(binary: &mut BinaryIter) -> Vec<Packet> {
        let number = binary.take_and_convert(NUMBER_SUB_PACKETS);

        (0..number).filter_map(|_| try_parse(binary)).collect()
    }

    fn new(version: u8, id: u8, binary: &mut BinaryIter) -> Self {
        let subpackets = match binary.next() {
            Some('0') => OperatorPacket::subpackets_by_length(binary),
            Some('1') => OperatorPacket::subpackets_by_number(binary),
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
        let operation = match self.id {
            0 => |a, b| a + b,
            1 => |a, b| a * b,
            2 => min,
            3 => max,
            5 => |a, b| u64::from(a > b),
            6 => |a, b| u64::from(a < b),
            7 => |a, b| u64::from(a == b),
            x => panic!("Unexpected id: {}", x),
        };

        self.subpackets
            .iter()
            .map(Packet::value)
            .reduce(operation)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn parse_with_literal_works() {
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
    fn parse_with_operator_works() {
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
    fn decode_binary_to_hex_works() {
        assert_eq!(
            decode_binary_to_hex(&['1', '0', '1', '0']),
            String::from("a")
        );
    }

    #[test]
    fn decode_nibbles_to_hex_works() {
        assert_eq!(
            decode_nibbles_to_hex(&[
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
        assert_eq!(part_two(&parse("04005AC33890")), 54);
        assert_eq!(part_two(&parse("880086C3E88112")), 7);
        assert_eq!(part_two(&parse("CE00C43D881120")), 9);
        assert_eq!(part_two(&parse("D8005AC2A8F0")), 1);
        assert_eq!(part_two(&parse("F600BC2D8F")), 0);
        assert_eq!(part_two(&parse("9C005AC2F8F0")), 0);
        assert_eq!(part_two(&parse("9C0141080250320F1802104A08")), 1);
    }
}

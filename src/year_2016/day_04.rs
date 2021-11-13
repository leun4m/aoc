use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn main(input: &str) {
    let rooms = parse(input);
    println!("Part 1: {}", part_one(&rooms));
    println!("Part 2: {}", part_two(&rooms));
}

fn parse(input: &str) -> Vec<Room> {
    input
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<Room>>()
}

fn part_one(rooms: &[Room]) -> u32 {
    rooms
        .iter()
        .filter(|room| room.is_real())
        .map(|room| room.id())
        .sum()
}

fn part_two(rooms: &[Room]) -> u32 {
    rooms
        .iter()
        .find(|room| room.decrypt().contains("northpole"))
        .unwrap()
        .id()
}

fn parse_line(line: &str) -> Room {
    let regex = Regex::new(r"([-a-z]+)-([0-9]+)\[(\w)(\w)(\w)(\w)(\w)\]").unwrap();
    let capture = regex.captures(line).expect("Looks weird");

    let name = capture[1].parse::<String>().unwrap();
    let id = capture[2].parse::<u32>().unwrap();
    let checksum = [
        capture[3].parse::<char>().unwrap(),
        capture[4].parse::<char>().unwrap(),
        capture[5].parse::<char>().unwrap(),
        capture[6].parse::<char>().unwrap(),
        capture[7].parse::<char>().unwrap(),
    ];

    Room::new(name.as_str(), id, checksum)
}

#[derive(Debug, PartialEq, Eq)]
struct Room {
    name: String,
    id: u32,
    checksum: [char; 5],
}

impl Room {
    pub fn new(name: &str, id: u32, checksum: [char; 5]) -> Room {
        Room {
            name: name.into(),
            id,
            checksum,
        }
    }

    pub fn is_real(&self) -> bool {
        let ranking = get_ranking(&self.name);
        if ranking.len() < 5 {
            panic!("Unexpected count of characters: {}", ranking.len());
        }

        ranking[0] == self.checksum[0]
            && ranking[1] == self.checksum[1]
            && ranking[2] == self.checksum[2]
            && ranking[3] == self.checksum[3]
            && ranking[4] == self.checksum[4]
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn decrypt(&self) -> String {
        let shift = (self.id % 26) as u8;
        self.name
            .to_ascii_lowercase()
            .chars()
            .map(|x| {
                if x == '-' {
                    return ' ';
                }

                let a = ((x as u8) + shift) as char;
                if 'z' < a {
                    ((a as u8) - 26) as char
                } else {
                    a
                }
            })
            .collect()
    }
}

fn count_chars(text: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();

    for c in text.chars().filter(|x| *x != '-') {
        let counter = map.get(&c).unwrap_or(&0);
        map.insert(c, counter + 1);
    }

    map
}

fn get_ranking(text: &str) -> Vec<char> {
    let map = count_chars(text);

    let chars = text
        .chars()
        .filter(|x| *x != '-')
        .unique()
        .sorted()
        .collect::<Vec<char>>();
    let counts = map
        .values()
        .copied()
        .unique()
        .sorted_by(|a, b| b.cmp(a))
        .collect::<Vec<i32>>();

    let mut vec = Vec::new();
    for i in counts {
        for c in &chars {
            if Some(&(i as i32)) == map.get(c) {
                vec.push(*c);
                if vec.len() >= 5 {
                    return vec;
                }
            }
        }
    }

    vec
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_works() {
        assert_eq!(
            parse_line("aaaaa-bbb-z-y-x-123[abxyz]"),
            Room::new("aaaaa-bbb-z-y-x", 123, ['a', 'b', 'x', 'y', 'z'])
        );
        assert_eq!(
            parse_line("a-b-c-d-e-f-g-h-987[abcde]"),
            Room::new("a-b-c-d-e-f-g-h", 987, ['a', 'b', 'c', 'd', 'e'])
        );
        assert_eq!(
            parse_line("not-a-real-room-404[oarel]"),
            Room::new("not-a-real-room", 404, ['o', 'a', 'r', 'e', 'l'])
        );
        assert_eq!(
            parse_line("totally-real-room-200[decoy]"),
            Room::new("totally-real-room", 200, ['d', 'e', 'c', 'o', 'y'])
        );
    }

    #[test]
    fn check_real_works() {
        assert!(Room::new("aaaaa-bbb-z-y-x", 123, ['a', 'b', 'x', 'y', 'z']).is_real());
        assert!(Room::new("a-b-c-d-e-f-g-h", 987, ['a', 'b', 'c', 'd', 'e']).is_real());
        assert!(Room::new("not-a-real-room", 404, ['o', 'a', 'r', 'e', 'l']).is_real());
        assert!(!Room::new("totally-real-room", 200, ['d', 'e', 'c', 'o', 'y']).is_real());
    }

    #[test]
    fn check_decrypt() {
        assert_eq!(
            parse_line("qzmt-zixmtkozy-ivhz-343[abcde]").decrypt(),
            "very encrypted name"
        );
    }
}

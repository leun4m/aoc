use regex::Regex;

pub fn main(input: &str) {
    let cookies = parse(input);

    println!("Part 1: {}", part_one(&cookies));
    println!("Part 2: {}", part_two(&cookies));
}

fn parse(input: &str) -> Vec<Cookie> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_cookie(line.trim()))
        .collect()
}

fn part_one(cookies: &[Cookie]) -> i32 {
    0
}

fn part_two(cookies: &[Cookie]) -> i32 {
    0
}

fn parse_cookie(input: &str) -> Cookie {
    // Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    let captures = Regex::new(r#"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)"#).unwrap()
    .captures(input).expect("Looks weird");

    let name: String = captures[1].parse().unwrap();
    let capacity: i32 = captures[2].parse().unwrap();
    let durability: i32 = captures[3].parse().unwrap();
    let flavor: i32 = captures[4].parse().unwrap();
    let texture: i32 = captures[5].parse().unwrap();
    let calories: i32 = captures[6].parse().unwrap();

    Cookie::new(name, capacity, durability, flavor, texture, calories)
}

#[derive(Debug, PartialEq, Eq)]
struct Cookie {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Cookie {
    pub fn new(
        name: String,
        capacity: i32,
        durability: i32,
        flavor: i32,
        texture: i32,
        calories: i32,
    ) -> Self {
        Self {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_cookie_works() {
        assert_eq!(
            parse_cookie(
                "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8"
            ),
            Cookie::new("Butterscotch".into(), -1, -2, 6, 3, 8)
        );
        assert_eq!(
            parse_cookie(
                "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
            ),
            Cookie::new("Cinnamon".into(), 2, 3, -2, -1, 3)
        );
    }
}

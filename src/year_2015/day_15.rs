use regex::Regex;
use std::cmp;

pub fn solve(input: &str) {
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

const TEASPOON_TOTAL: i64 = 100;
const CALORIES_TOTAL: i64 = 500;

fn part_one(cookies: &[Cookie]) -> i64 {
    all_ratios(cookies.len(), TEASPOON_TOTAL)
        .iter()
        .map(|ratio| test_score(cookies, ratio))
        .max()
        .unwrap_or_default()
}

fn part_two(cookies: &[Cookie]) -> i64 {
    all_ratios(cookies.len(), TEASPOON_TOTAL)
        .iter()
        .map(|ratio| test_score_with_calories(cookies, ratio, CALORIES_TOTAL))
        .max()
        .unwrap_or_default()
}

fn test_score(cookies: &[Cookie], teaspoons: &[i64]) -> i64 {
    if cookies.len() != teaspoons.len() {
        println!("Cookies and teaspoons must have the same length");
        0
    } else {
        sum_all(cookies, teaspoons)
    }
}

fn test_score_with_calories(cookies: &[Cookie], teaspoons: &[i64], calories_wanted: i64) -> i64 {
    if cookies.len() != teaspoons.len() {
        println!("Cookies and teaspoons must have the same length");
        0
    } else {
        let calories = sum_properties(cookies, teaspoons, |c| c.calories);

        if calories == calories_wanted {
            sum_all(cookies, teaspoons)
        } else {
            0
        }
    }
}

fn sum_all(cookies: &[Cookie], teaspoons: &[i64]) -> i64 {
    let c = sum_properties(cookies, teaspoons, |c| c.capacity);
    let d = sum_properties(cookies, teaspoons, |c| c.durability);
    let f = sum_properties(cookies, teaspoons, |c| c.flavor);
    let t = sum_properties(cookies, teaspoons, |c| c.texture);
    c * d * f * t
}

fn all_ratios(count: usize, total: i64) -> Vec<Vec<i64>> {
    let mut result = Vec::new();

    for i in 0..(total + 1) {
        let mut line = Vec::new();
        if count > 1 {
            for subratio in all_ratios(count - 1, total - i).iter_mut() {
                line.push(i);
                line.append(subratio);
                result.push(line.clone());
                line.clear();
            }
        } else if i == total {
            result.push(vec![i]);
        }
    }

    result
}

type PropertyOp = fn(&Cookie) -> i64;

fn sum_properties(cookies: &[Cookie], teaspoons: &[i64], f: PropertyOp) -> i64 {
    cmp::max(
        0,
        cookies
            .iter()
            .map(|cookie| f(cookie))
            .zip(teaspoons)
            .map(|(property, teaspoon)| property * teaspoon)
            .sum(),
    )
}

fn parse_cookie(input: &str) -> Cookie {
    // Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    let captures = Regex::new(r#"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)"#).unwrap()
    .captures(input).expect("Looks weird");

    let name: String = captures[1].parse().unwrap();
    let capacity: i64 = captures[2].parse().unwrap();
    let durability: i64 = captures[3].parse().unwrap();
    let flavor: i64 = captures[4].parse().unwrap();
    let texture: i64 = captures[5].parse().unwrap();
    let calories: i64 = captures[6].parse().unwrap();

    Cookie::new(name, capacity, durability, flavor, texture, calories)
}

#[derive(Debug, PartialEq, Eq)]
struct Cookie {
    pub name: String,
    pub capacity: i64,
    pub durability: i64,
    pub flavor: i64,
    pub texture: i64,
    pub calories: i64,
}

impl Cookie {
    pub fn new(
        name: String,
        capacity: i64,
        durability: i64,
        flavor: i64,
        texture: i64,
        calories: i64,
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
            parse_cookie("Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"),
            Cookie::new("Cinnamon".into(), 2, 3, -2, -1, 3)
        );
    }

    #[test]
    fn test_score_works() {
        let cookies = vec![
            Cookie::new("Butterscotch".into(), -1, -2, 6, 3, 8),
            Cookie::new("Cinnamon".into(), 2, 3, -2, -1, 3),
        ];

        assert_eq!(test_score(&cookies, &vec![44, 56]), 62842880);
    }

    #[test]
    fn all_ratios_works() {
        assert_eq!(
            all_ratios(2, 10),
            vec![
                vec![0, 10],
                vec![1, 9],
                vec![2, 8],
                vec![3, 7],
                vec![4, 6],
                vec![5, 5],
                vec![6, 4],
                vec![7, 3],
                vec![8, 2],
                vec![9, 1],
                vec![10, 0]
            ]
        );
    }
}

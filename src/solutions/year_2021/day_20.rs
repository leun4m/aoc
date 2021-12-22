use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::HashSet;

pub fn solve(input: &str) {
    let (algorithm, image) = parse(input);
    println!("Part 1: {}", part_one(&algorithm, &Image::new(image)));
}

fn parse(input: &str) -> (Algorithm, Vec<Vec<Pixel>>) {
    let algorithm = input
        .lines()
        .map(|line| line.trim())
        .find(|line| !line.is_empty())
        .unwrap()
        .chars()
        .map(Pixel::from)
        .collect::<Vec<Pixel>>()
        .try_into()
        .unwrap();
    let image = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .skip(1)
        .map(|line| parse_line(line))
        .collect();

    (algorithm, image)
}

fn part_one(algorithm: &Algorithm, image: &Image) -> usize {
    let mut new_image = image.clone();

    for _ in 0..2 {
        new_image = enhance(algorithm, &new_image);
    }

    new_image.print();

    new_image.count_lit()
}

fn enhance(algorithm: &Algorithm, image: &Image) -> Image {
    let mut new_image = Image::empty();
    for x in (image.min_x - 1)..=(image.max_x + 1) {
        for y in (image.min_y - 1)..=(image.max_y + 1) {
            let v = image.pixel_val(x, y);
            let w = convert_pixels(v);
            if w != 0 && algorithm[w] == Pixel::Light {
                new_image.add_pixel(x, y);
            }
        }
    }

    new_image
}

fn parse_line(line: &str) -> Vec<Pixel> {
    line.chars().map(Pixel::from).collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Dark,
    Light,
}

impl Pixel {
    fn binary(&self) -> char {
        match self {
            Pixel::Dark => '0',
            Pixel::Light => '1',
        }
    }
}

impl From<char> for Pixel {
    fn from(c: char) -> Pixel {
        match c {
            '.' => Pixel::Dark,
            '#' => Pixel::Light,
            _ => panic!("unexpected char!"),
        }
    }
}

impl std::fmt::Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Pixel::Dark => write!(f, "."),
            Pixel::Light => write!(f, "#"),
        }
    }
}

#[derive(Clone)]
struct Image {
    lights: HashSet<(i32, i32)>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Image {
    fn empty() -> Self {
        Self {
            lights: HashSet::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }

    fn new(pixels: Vec<Vec<Pixel>>) -> Self {
        let mut image = Self::empty();

        for y in 0..pixels.len() {
            for x in 0..pixels[y].len() {
                if pixels[y][x] == Pixel::Light {
                    image.lights.insert((x as i32, y as i32));
                }
            }
        }

        image.update_size();
        image
    }

    fn pixel(&self, x: i32, y: i32) -> Pixel {
        if self.lights.contains(&(x, y)) {
            Pixel::Light
        } else {
            Pixel::Dark
        }
    }

    fn pixel_val(&self, x: i32, y: i32) -> [Pixel; 9] {
        [
            self.pixel(x - 1, y - 1),
            self.pixel(x, y - 1),
            self.pixel(x + 1, y - 1),
            self.pixel(x - 1, y),
            self.pixel(x, y),
            self.pixel(x + 1, y),
            self.pixel(x - 1, y + 1),
            self.pixel(x, y + 1),
            self.pixel(x + 1, y + 1),
        ]
    }

    fn add_pixel(&mut self, x: i32, y: i32) {
        self.lights.insert((x, y));
        self.update_size();
    }

    fn update_size(&mut self) {
        let (min_x, max_x) = if let MinMax(min, max) = self.lights.iter().map(|(x, _)| x).minmax() {
            (*min, *max)
        } else {
            (0, 0)
        };

        let (min_y, max_y) = if let MinMax(min, max) = self.lights.iter().map(|(_, y)| y).minmax() {
            (*min, *max)
        } else {
            (0, 0)
        };

        self.min_x = min_x;
        self.min_y = min_y;
        self.max_x = max_x;
        self.max_y = max_y;
    }

    fn count_lit(&self) -> usize {
        self.lights.len()
    }

    fn print(&self) {
        println!("///////////////////");
        println!(
            "x: {} - {} y: {} - {}",
            self.min_x, self.max_x, self.min_y, self.max_y
        );
        println!("///////////////////");
        for y in (self.min_y - 1)..=(self.max_y + 1) {
            for x in (self.min_x - 1)..=(self.max_x + 1) {
                print!("{:?}", self.pixel(x, y));
            }
            println!();
        }
        println!("///////////////////");
    }
}

fn convert_pixels(pixels: [Pixel; 9]) -> usize {
    usize::from_str_radix(&pixels.iter().map(|p| p.binary()).collect::<String>(), 2).unwrap()
}

type Algorithm = [Pixel; 512];

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "
    ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

    #..#.
    #....
    ##..#
    ..#..
    ..###
    ";

    #[test]
    fn part_one_works() {
        let (algorithm, image) = parse(INPUT);
        let result = part_one(&algorithm, &Image::new(image));
        assert_eq!(result, 35);
    }
}

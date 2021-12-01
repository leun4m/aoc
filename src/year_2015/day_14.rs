use regex::Regex;

const TIME: u32 = 2503;

pub fn solve(input: &str) {
    let mut reindeers = parse(input);

    println!("Part 1: {}", part_one(&mut reindeers));

    for reindeer in reindeers.iter_mut() {
        reindeer.reset();
    }

    println!("Part 2: {}", part_two(&mut reindeers));
}

fn parse(input: &str) -> Vec<Reindeer> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: &str) -> Reindeer {
    // "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."
    let regex = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();
    let capture = regex.captures(line).expect("Looks weird");

    let name = capture[1].parse::<String>().unwrap();
    let speed = capture[2].parse::<u32>().unwrap();
    let speed_time = capture[3].parse::<u32>().unwrap();
    let rest_time = capture[4].parse::<u32>().unwrap();
    Reindeer::new(&name, speed, speed_time, rest_time)
}

fn part_one(reindeers: &mut [Reindeer]) -> u32 {
    for _ in 0..TIME {
        for reindeer in reindeers.iter_mut() {
            reindeer.tick()
        }
    }
    reindeers.iter().map(|r| r.distance()).max().unwrap()
}

fn part_two(reindeers: &mut [Reindeer]) -> u32 {
    for _ in 0..TIME {
        for reindeer in reindeers.iter_mut() {
            reindeer.tick()
        }

        let first = reindeers.iter_mut().max_by_key(|r| r.distance()).unwrap();
        first.add_point();
    }
    reindeers.iter().map(|r| r.score()).max().unwrap()
}

#[derive(Debug, PartialEq, Eq)]
struct Reindeer {
    name: String,
    speed: u32,
    speed_time: u32,
    rest_time: u32,
    current_state: State,
    current_time: u32,
    distance: u32,
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Flying,
    Resting,
}

impl Reindeer {
    pub fn new(name: &str, speed: u32, speed_time: u32, rest_time: u32) -> Self {
        Self {
            name: name.to_string(),
            speed,
            speed_time,
            rest_time,
            current_state: State::Resting,
            current_time: 0,
            distance: 0,
            score: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.current_time == 0 {
            self.switch_state();
        }

        if self.current_state == State::Flying {
            self.distance += self.speed;
        }

        self.current_time -= 1;
    }

    fn switch_state(&mut self) {
        match &self.current_state {
            State::Resting => {
                self.current_state = State::Flying;
                self.current_time = self.speed_time;
            }
            State::Flying => {
                self.current_state = State::Resting;
                self.current_time = self.rest_time;
            }
        }
    }

    pub fn distance(&self) -> u32 {
        self.distance
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn reset(&mut self) {
        self.distance = 0;
        self.current_state = State::Resting;
        self.current_time = 0;
        self.score = 0;
    }

    pub fn add_point(&mut self) {
        self.score += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_works() {
        assert_eq!(
            parse_line("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."),
            Reindeer::new("Comet", 14, 10, 127)
        );
        assert_eq!(
            parse_line(
                "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
            ),
            Reindeer::new("Dancer", 16, 11, 162)
        );
    }
}

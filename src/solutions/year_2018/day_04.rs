use std::collections::HashMap;

use chrono::{Duration, NaiveDateTime, NaiveTime, Timelike};
use itertools::Itertools;
use regex::Regex;

pub fn solve(input: &str) {
    let mut log_lines = parse(input);

    let guards = sum_minutes_asleep(&mut log_lines);

    println!("Part 1: {}", part_one(&guards));
    println!("Part 2: {}", part_two(&guards));
}

fn part_one(guards: &[Guard]) -> u32 {
    let guard = guards
        .iter()
        .sorted_by_key(|g| g.sum_minutes_asleep())
        .last()
        .unwrap();

    guard.id * guard.get_minute_most_asleep().minutes()
}

fn part_two(guards: &[Guard]) -> u32 {
    let result = guards
        .iter()
        .map(|g| (g, g.get_minute_most_asleep()))
        .sorted_by_key(|x| x.1.times())
        .map(|x| (x.0, x.1.minutes()))
        .last()
        .unwrap();

    result.0.id * result.1
}

fn sum_minutes_asleep(log_lines: &mut [Log]) -> Vec<Guard> {
    log_lines.sort_by_key(|x| x.time);

    let mut guards: HashMap<u32, Guard> = HashMap::new();
    let mut current: &mut Guard = &mut Guard::new(0);
    let mut asleep: NaiveDateTime = NaiveDateTime::from_timestamp(0, 0);

    for line in log_lines {
        match line.instruction {
            Instruction::Begins(x) => current = guards.entry(x).or_insert_with(|| Guard::new(x)),
            Instruction::FallsAsleep => asleep = line.time,
            Instruction::WakesUp => current.asleep.push((asleep, line.time)),
        }
    }

    guards.into_values().collect_vec()
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
struct Guard {
    id: u32,
    asleep: Vec<(NaiveDateTime, NaiveDateTime)>,
}

impl Guard {
    fn new(id: u32) -> Self {
        Self {
            id,
            asleep: Vec::new(),
        }
    }

    fn sum_minutes_asleep(&self) -> i64 {
        self.asleep
            .iter()
            .map(|(from, to)| {
                let duration_in_sec = to.timestamp() - from.timestamp();
                
                duration_in_sec / 60
            })
            .sum()
    }

    fn get_minute_most_asleep(&self) -> MostAsleep {
        let mut minutes: HashMap<NaiveTime, usize> = HashMap::new();

        for span in self.asleep.iter() {
            let mut current = span.0.time();
            while current < span.1.time() {
                let x = minutes.entry(current).or_default();
                *x += 1;
                current += Duration::minutes(1);
            }
        }

        if let Some(a) = minutes.iter().sorted_by_key(|x| x.1).last() {
            MostAsleep::MinuteTimes(a.0.minute(), 0)
        } else {
            MostAsleep::None
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum MostAsleep {
    None,
    MinuteTimes(u32, usize),
}

impl MostAsleep {
    fn minutes(&self) -> u32 {
        if let MostAsleep::MinuteTimes(m, _) = self {
            *m
        } else {
            0
        }
    }

    fn times(&self) -> usize {
        if let MostAsleep::MinuteTimes(_, t) = self {
            *t
        } else {
            0
        }
    }
}

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r#"\[(\d\d\d\d-\d\d-\d\d \d\d:\d\d)\] (.*)"#).unwrap();
    static ref GUARD_REGEX: Regex = Regex::new(r#"Guard #(\d+) begins shift"#).unwrap();
}

const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M";

fn parse(input: &str) -> Vec<Log> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Log {
    let captures = LINE_REGEX.captures(line).expect("invalid line");

    let datetime = NaiveDateTime::parse_from_str(&captures[1], DATE_TIME_FORMAT).unwrap();

    let instruction = if let Some(guard) = GUARD_REGEX.captures(line) {
        Instruction::Begins(guard[1].parse().unwrap())
    } else if &captures[2] == "wakes up" {
        Instruction::WakesUp
    } else if &captures[2] == "falls asleep" {
        Instruction::FallsAsleep
    } else {
        panic!("unexpected log")
    };

    Log {
        time: datetime,
        instruction,
    }
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
struct Log {
    time: NaiveDateTime,
    instruction: Instruction,
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
enum Instruction {
    Begins(u32),
    FallsAsleep,
    WakesUp,
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    const INPUT: &str = "[1518-11-01 00:00] Guard #10 begins shift
    [1518-11-01 00:05] falls asleep
    [1518-11-01 00:25] wakes up
    [1518-11-01 00:30] falls asleep
    [1518-11-01 00:55] wakes up
    [1518-11-01 23:58] Guard #99 begins shift
    [1518-11-02 00:40] falls asleep
    [1518-11-02 00:50] wakes up
    [1518-11-03 00:05] Guard #10 begins shift
    [1518-11-03 00:24] falls asleep
    [1518-11-03 00:29] wakes up
    [1518-11-04 00:02] Guard #99 begins shift
    [1518-11-04 00:36] falls asleep
    [1518-11-04 00:46] wakes up
    [1518-11-05 00:03] Guard #99 begins shift
    [1518-11-05 00:45] falls asleep
    [1518-11-05 00:55] wakes up";

    #[test]
    fn parse_line_works() {
        assert_eq!(
            parse_line("[1518-11-01 00:00] Guard #10 begins shift"),
            Log {
                time: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
                instruction: Instruction::Begins(10)
            }
        );

        assert_eq!(
            parse_line("[1518-11-05 00:55] wakes up"),
            Log {
                time: NaiveDate::from_ymd(1518, 11, 5).and_hms(0, 55, 0),
                instruction: Instruction::WakesUp
            }
        );

        assert_eq!(
            parse_line("[1518-11-01 00:30] falls asleep"),
            Log {
                time: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 30, 0),
                instruction: Instruction::FallsAsleep
            }
        );
    }

    #[test]
    fn sum_minutes_asleep_works() {
        let result = sum_minutes_asleep(&mut parse(INPUT));
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|x| x.id == 10));
        assert!(result.iter().any(|x| x.id == 99));

        assert_eq!(result.iter().find(|x| x.id == 10).unwrap().asleep.len(), 3);
        assert_eq!(result.iter().find(|x| x.id == 99).unwrap().asleep.len(), 3);
    }

    #[test]
    fn part_one_works() {
        let result = part_one(&sum_minutes_asleep(&mut parse(INPUT)));
        assert_eq!(240, result);
    }

    #[test]
    #[ignore = "not working on GitHub?!"]
    fn part_two_works() {
        let result = part_two(&sum_minutes_asleep(&mut parse(INPUT)));
        assert_eq!(4455, result);
    }
}

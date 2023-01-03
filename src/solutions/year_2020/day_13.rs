pub fn solve(input: &str) {
    let (earliest, times) = parse_input(input);
    let bus = part_one(&times, earliest);
    let timestamp = part_two(&times);
    println!("Part 1: {}", calc_result(bus, earliest));
    println!("Part 2: {timestamp}");
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum BusTime {
    X,
    Id(u64),
}

#[derive(Clone, Copy)]
struct Bus {
    id: u64,
    departure: u64,
}

/// Inspired by <https://github.com/azablan/advent-of-code-2020/blob/main/walkthrough/d13/part2.js>
fn part_two(times: &[BusTime]) -> u64 {
    let indexed_times = times
        .iter()
        .enumerate()
        .filter(|(_, &x)| x != BusTime::X)
        .map(|(i, x)| {
            if let BusTime::Id(id) = x {
                (i as u64, *id)
            } else {
                panic!("...")
            }
        })
        .collect::<Vec<(u64, u64)>>();

    let mut time = 0;
    let mut step_size = indexed_times.first().unwrap().1;

    for (i, bus) in indexed_times.iter().skip(1) {
        while (time + i) % bus != 0 {
            time += step_size;
        }

        step_size *= bus;
    }
    time
}

fn part_one(times: &[BusTime], earliest: u64) -> Bus {
    times
        .iter()
        .filter(|&&x| x != BusTime::X)
        .map(|x| {
            if let BusTime::Id(id) = x {
                let mut a = 0;
                loop {
                    a += id;
                    if a > earliest {
                        break;
                    }
                }
                Bus {
                    id: *id,
                    departure: a,
                }
            } else {
                panic!("Shouldn't be here!");
            }
        })
        .min_by_key(|x| x.departure)
        .expect("No minimum found!")
}

fn calc_result(bus: Bus, earliest: u64) -> u64 {
    (bus.departure - earliest) * bus.id
}

fn parse_input(input: &str) -> (u64, Vec<BusTime>) {
    let mut lines = input.lines();
    let earliest = lines.next().unwrap().parse::<u64>().unwrap();
    let times = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| {
            if x == "x" {
                BusTime::X
            } else {
                BusTime::Id(x.parse().unwrap())
            }
        })
        .collect::<Vec<BusTime>>();
    (earliest, times)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "939\n7,13,x,x,59,x,31,19";
        let (earliest, times) = parse_input(input);
        let bus = part_one(&times, earliest);
        assert_eq!(59, bus.id);
        assert_eq!(944, bus.departure);
        assert_eq!(295, calc_result(bus, earliest));
    }

    #[test]
    fn example2() {
        assert_eq!(3417, part_two(&parse_input("0\n17,x,13,19").1));
        assert_eq!(754_018, part_two(&parse_input("0\n67,7,59,61").1));
        assert_eq!(779_210, part_two(&parse_input("0\n67,x,7,59,61").1));
        assert_eq!(1_261_476, part_two(&parse_input("0\n67,7,x,59,61").1));
        assert_eq!(
            1_202_161_486,
            part_two(&parse_input("0\n1789,37,47,1889").1)
        );
    }
}

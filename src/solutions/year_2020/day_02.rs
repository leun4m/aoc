use regex::Regex;

pub fn solve(input: &str) {
    let regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut valid_1 = 0;
    let mut valid_2 = 0;

    for line in input.lines() {
        let capture = regex.captures(line).expect("Looks weird");
        let min = capture[1].parse::<usize>().unwrap();
        let max = capture[2].parse::<usize>().unwrap();
        let chr = capture[3].chars().next().unwrap();
        let word = capture[4].to_string();
        if is_valid_1(min, max, chr, &word) {
            valid_1 += 1;
        }
        if is_valid_2(min, max, chr, &word) {
            valid_2 += 1;
        }
    }
    println!("Part 1: {valid_1}");
    println!("Part 2: {valid_2}");
}

fn is_valid_2(min: usize, max: usize, chr: char, password: &str) -> bool {
    let chr_min = password.chars().nth(min - 1).unwrap();
    let chr_max = password.chars().nth(max - 1).unwrap();

    (chr_min == chr && chr_max != chr) || (chr_min != chr && chr_max == chr)
}

fn is_valid_1(min: usize, max: usize, char: char, password: &str) -> bool {
    let count = password.chars().filter(|c| c == &char).count();

    min <= count && count <= max
}

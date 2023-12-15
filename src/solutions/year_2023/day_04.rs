pub fn solve(input: &str) {
    let cards = parse(input);
    println!("Part 1: {}", part_one(&cards));
    println!("Part 2: {}", part_two(&cards));
}

fn parse(input: &str) -> Vec<Card> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Card {
    let parts_colon: Vec<&str> = line.split(':').collect();
    let parts_pipe: Vec<&str> = parts_colon[1].split('|').collect();

    let winning_numbers = parts_pipe[0]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let own_numbers = parts_pipe[1]
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    Card {
        winning_numbers,
        own_numbers,
    }
}

fn part_one(cards: &[Card]) -> u32 {
    cards
        .iter()
        .map(Card::count_winning_numbers)
        .filter(|&c| c > 0)
        .map(|x| 2_u32.pow((x - 1) as u32))
        .sum()
}

fn part_two(cards: &[Card]) -> usize {
    let mut card_count = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let winnings_cards = card.count_winning_numbers();

        for j in (i + 1)..(i + 1 + winnings_cards) {
            if j < card_count.len() {
                card_count[j] += card_count[i];
            }
        }
    }

    card_count.iter().sum()
}

struct Card {
    winning_numbers: Vec<u32>,
    own_numbers: Vec<u32>,
}

impl Card {
    fn count_winning_numbers(&self) -> usize {
        self.own_numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(x))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_one() {
        assert_eq!(13, part_one(&parse(EXAMPLE_INPUT)));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(30, part_two(&parse(EXAMPLE_INPUT)));
    }
}

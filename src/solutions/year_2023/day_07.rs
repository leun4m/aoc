use std::cmp::Ordering;

use itertools::Itertools;

use crate::parser;

pub fn solve(input: &str) {
    let hands = parse(input);
    println!("Part 1: {}", part_one(&hands));
    println!("Part 2: {}", part_two(&hands));
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    hand: Vec<CardLabel>,
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.rank().cmp(&other.rank());

        if ord != Ordering::Equal {
            return ord;
        }

        for i in 0..self.hand.len() {
            let ord_card = self.hand[i].cmp(&other.hand[i]);
            if ord_card != Ordering::Equal {
                return ord_card;
            }
        }

        Ordering::Equal
    }
}

impl Hand {
    fn rank(&self) -> usize {
        let counted = self.count_types();
        let jokers = counted
            .iter()
            .find(|x| x.0.is_joker())
            .map(|x| x.1)
            .unwrap_or_default();

        if Self::is_x_of_a_kind(&counted, jokers, 5) {
            6
        } else if Self::is_x_of_a_kind(&counted, jokers, 4) {
            5
        } else if Self::is_a_house(&counted, jokers, 3, 2) {
            4
        } else if Self::is_x_of_a_kind(&counted, jokers, 3) {
            3
        } else if Self::is_a_house(&counted, jokers, 2, 2) {
            2
        } else if Self::is_x_of_a_kind(&counted, jokers, 2) {
            1
        } else {
            0
        }
    }

    fn count_types(&self) -> Vec<(CardLabel, usize)> {
        CARD_LABELS
            .iter()
            .map(|&c| (c, self.hand.iter().filter(|&&x| x == c).count()))
            .collect()
    }

    fn switch_jack_to_joker(&self) -> Self {
        Self {
            hand: self.hand.iter().map(|x| x.switch_jack_to_joker()).collect(),
            bid: self.bid,
        }
    }

    fn is_x_of_a_kind(counted: &[(CardLabel, usize)], jokers: usize, count: usize) -> bool {
        counted.iter().any(|x| x.1 + jokers == count)
    }

    fn is_a_house(counted: &[(CardLabel, usize)], jokers: usize, a: usize, b: usize) -> bool {
        let two_most_cards: Vec<usize> = counted
            .iter()
            .filter(|x| !x.0.is_joker())
            .map(|x| x.1)
            .sorted()
            .rev()
            .take(2)
            .collect();

        if two_most_cards[0] + jokers >= a {
            let used_jokers = a - two_most_cards[0];

            return two_most_cards[1] + used_jokers >= b;
        }

        false
    }
}

const CARD_LABELS: [CardLabel; 14] = [
    CardLabel::Joker,
    CardLabel::Two,
    CardLabel::Three,
    CardLabel::Four,
    CardLabel::Five,
    CardLabel::Six,
    CardLabel::Seven,
    CardLabel::Eight,
    CardLabel::Nine,
    CardLabel::Ten,
    CardLabel::Jack,
    CardLabel::Queen,
    CardLabel::King,
    CardLabel::Ace,
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CardLabel {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardLabel {
    fn from(input: char) -> Self {
        match input {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Unexpected char"),
        }
    }

    fn switch_jack_to_joker(self) -> Self {
        match self {
            Self::Jack => Self::Joker,
            x => x,
        }
    }

    fn is_joker(self) -> bool {
        self == Self::Joker
    }
}

fn parse(input: &str) -> Vec<Hand> {
    parser::lines_custom(input, |line| {
        let splitted = line.split(' ').collect_vec();

        Hand {
            hand: splitted[0].chars().map(CardLabel::from).collect(),
            bid: splitted[1].parse().unwrap(),
        }
    })
}

fn part_one(hands: &[Hand]) -> usize {
    hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

fn part_two(hands: &[Hand]) -> usize {
    hands
        .iter()
        .map(Hand::switch_jack_to_joker)
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(EXAMPLE_INPUT)), 6440);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&parse(EXAMPLE_INPUT)), 5905);
    }

    #[test]
    fn test_rank() {
        let vec = parse(EXAMPLE_INPUT);
        assert_eq!(vec[0].rank(), 1);
        assert_eq!(vec[1].rank(), 3);
        assert_eq!(vec[2].rank(), 2);
        assert_eq!(vec[3].rank(), 2);
        assert_eq!(vec[4].rank(), 3);
    }
}

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
        let stats = &HandStats::from(self);
        HAND_TYPES
            .iter()
            .rev()
            .find(|x| x.is_this(stats))
            .map(HandType::rank)
            .unwrap()
    }

    fn switch_jack_to_joker(&self) -> Self {
        Self {
            hand: self.hand.iter().map(|x| x.switch_jack_to_joker()).collect(),
            bid: self.bid,
        }
    }
}

struct HandStats {
    card_times: Vec<(CardLabel, usize)>,
    jokers: usize,
}

impl HandStats {
    fn from(hand: &Hand) -> Self {
        let card_times = Self::count_types(hand);
        let jokers = card_times
            .iter()
            .filter(|x| x.0.is_joker())
            .map(|x| x.1)
            .sum();

        HandStats {
            card_times: card_times
                .into_iter()
                .filter(|x| !x.0.is_joker())
                .collect_vec(),
            jokers,
        }
    }

    fn count_types(hand: &Hand) -> Vec<(CardLabel, usize)> {
        CARD_LABELS
            .iter()
            .map(|&c| (c, hand.hand.iter().filter(|&&x| x == c).count()))
            .collect()
    }
}

const HAND_TYPES: [HandType; 7] = [
    HandType::HighCard,
    HandType::OnePair,
    HandType::TwoPair,
    HandType::ThreeOfAKind,
    HandType::FullHouse,
    HandType::FourOfAKind,
    HandType::FiveOfAKind,
];

enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn rank(&self) -> usize {
        match self {
            Self::HighCard => 1,
            Self::OnePair => 2,
            Self::TwoPair => 3,
            Self::ThreeOfAKind => 4,
            Self::FullHouse => 5,
            Self::FourOfAKind => 6,
            Self::FiveOfAKind => 7,
        }
    }

    fn is_this(&self, stats: &HandStats) -> bool {
        match self {
            Self::HighCard => true,
            Self::OnePair => Self::is_x_of_a_kind(stats, 2),
            Self::ThreeOfAKind => Self::is_x_of_a_kind(stats, 3),
            Self::FourOfAKind => Self::is_x_of_a_kind(stats, 4),
            Self::FiveOfAKind => Self::is_x_of_a_kind(stats, 5),
            Self::TwoPair => Self::is_a_house(stats, 2, 2),
            Self::FullHouse => Self::is_a_house(stats, 3, 2),
        }
    }

    fn is_x_of_a_kind(stats: &HandStats, count: usize) -> bool {
        stats
            .card_times
            .iter()
            .map(|x| x.1)
            .any(|x| x + stats.jokers == count)
    }

    fn is_a_house(stats: &HandStats, a: usize, b: usize) -> bool {
        let two_most_cards: Vec<usize> = stats
            .card_times
            .iter()
            .filter(|x| !x.0.is_joker())
            .map(|x| x.1)
            .sorted()
            .rev()
            .take(2)
            .collect();

        if two_most_cards[0] + stats.jokers >= a {
            let used_jokers = a - two_most_cards[0];

            return two_most_cards[1] + (stats.jokers - used_jokers) >= b;
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
}

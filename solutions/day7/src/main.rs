use crate::Score::{FiveOfKind, FourOfKind, FullHouse, HighCard, OnePair, ThreeOfKind, TwoPair};
use anyhow::{anyhow, Result};
use inputs::{read_lines, Lines};
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines = read_lines("day7.txt")?;

    println!("Day Seven");
    println!("-------");
    println!("part one: {}", part_one(&lines)?);
    println!("part two: {}", part_two(&lines)?);

    Ok(())
}

fn part_one(lines: &Lines) -> Result<usize> {
    calculate_total_winnings(lines, false)
}

fn part_two(lines: &Lines) -> Result<usize> {
    calculate_total_winnings(lines, true)
}

fn calculate_total_winnings(lines: &Lines, jokers: bool) -> Result<usize> {
    let cards = if !jokers { CARDS_1 } else { CARDS_2 };

    let mut hands = lines
        .iter()
        .flat_map(|l| Hand::from_str(l, jokers))
        .collect::<Vec<Hand>>();

    sort(&mut hands, cards);

    let total_hands = hands.len();
    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + (h.bid * (total_hands - i)));

    Ok(total_winnings)
}

const CARDS_1: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const CARDS_2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn sort(hands: &mut Vec<Hand>, cards: [char; 13]) {
    hands.sort_by(|h1, h2| {
        if h1.score > h2.score {
            Ordering::Greater
        } else if h1.score < h2.score {
            Ordering::Less
        } else {
            for i in 0..h1.cards.len() {
                let h1c = cards.iter().position(|c| c == &h1.cards[i]).unwrap();
                let h2c = cards.iter().position(|c| c == &h2.cards[i]).unwrap();

                if h1c > h2c {
                    return Ordering::Greater;
                } else if h1c < h2c {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        }
    })
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Score {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    score: Score,
    bid: usize,
}

impl Hand {
    fn from_str(s: &str, jokers: bool) -> Result<Self> {
        let mut iter = s.split_whitespace().into_iter();
        let cards = iter
            .next()
            .ok_or_else(|| anyhow!("Couldn't parse cards"))?
            .chars()
            .collect::<Vec<_>>();
        let bid = iter
            .next()
            .map(|s| s.parse::<usize>())
            .ok_or(anyhow!("Couldn't parse bid"))??;
        let score = Self::calc_score(&cards, jokers);

        Ok(Self { cards, bid, score })
    }

    fn calc_score(cards: &[char], jokers: bool) -> Score {
        let mut char_counts = HashMap::new();

        for c in cards {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        if jokers {
            let joker_count = *char_counts.entry(&'J').or_default();
            if joker_count > 0 {
                let mut highest_char = None;
                let mut highest_value = 0;

                for (&key, &value) in char_counts.iter() {
                    if value > highest_value && key != &'J' {
                        highest_value = value;
                        highest_char = Some(key);
                    }
                }

                match highest_char {
                    Some(c) => {
                        *char_counts.entry(c).or_insert(0) += joker_count;
                        char_counts.remove(&'J');
                    }
                    _ => {}
                };
            }
        }

        let mut counts: Vec<i32> = char_counts.values().cloned().collect();
        counts.sort_by(|c1, c2| c2.cmp(c1));

        match counts {
            c if c.starts_with(&[5]) => FiveOfKind,
            c if c.starts_with(&[4]) => FourOfKind,
            c if c.starts_with(&[3, 2]) => FullHouse,
            c if c.starts_with(&[3]) => ThreeOfKind,
            c if c.starts_with(&[2, 2]) => TwoPair,
            c if c.starts_with(&[2]) => OnePair,
            _ => HighCard,
        }
    }
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    #[test]
    fn part_1() {
        let lines = read_lines("day7.txt").unwrap();
        let answer = part_one(&lines).unwrap();
        assert_eq!(answer, 253954294);
    }

    #[test]
    fn part_2() {
        let lines = read_lines("day7.txt").unwrap();
        let answer = part_two(&lines).unwrap();
        assert_eq!(answer, 254837398);
    }
}

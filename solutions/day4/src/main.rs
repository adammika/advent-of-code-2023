use anyhow::{anyhow, Result};
use inputs::{read_lines, Lines};
use std::collections::HashSet;
use std::str::FromStr;

fn main() -> Result<()> {
    let lines = read_lines("day4.txt")?;

    println!("Day Four");
    println!("-------");
    println!("part one: {}", part_one(&lines)?);
    println!("part two: {}", part_two(&lines)?);

    Ok(())
}

fn part_one(lines: &Lines) -> Result<u32> {
    let sum = lines
        .iter()
        .map(|l| Card::from_str(l).unwrap())
        .map(|c| c.score())
        .sum();

    Ok(sum)
}

fn part_two(lines: &Lines) -> Result<u32> {
    let match_counts: Vec<usize> = lines
        .iter()
        .map(|line| Card::from_str(line).unwrap().matches().iter().count())
        .collect();

    let mut cards: Vec<usize> = vec![1; match_counts.len()];
    for (i, num_matches) in match_counts.iter().enumerate() {
        for j in i..i + num_matches {
            cards[j + 1] += cards[i];
        }
    }

    let total_cards = cards.iter().map(|&num| num as u32).sum();
    Ok(total_cards)
}

#[derive(Clone, Debug)]
struct Card {
    numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

impl Card {
    fn matches(&self) -> HashSet<u32> {
        self.winning_numbers
            .intersection(&self.numbers)
            .copied()
            .collect()
    }

    fn score(&self) -> u32 {
        let num_matches = self.matches().iter().count();
        if num_matches == 0 {
            0
        } else {
            1 << (num_matches - 1)
        }
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let _ = parts.next().ok_or(anyhow!("no id part"))?;
        let numbers_part = parts.next().ok_or(anyhow!("no numbers part"))?;

        let mut numbers_parts = numbers_part.split(" | ");

        let numbers: HashSet<u32> = numbers_parts
            .next()
            .ok_or(anyhow!("couldn't parse numbers"))?
            .trim()
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let winning_numbers: HashSet<u32> = numbers_parts
            .next()
            .ok_or(anyhow!("couldn't parse winning numbers"))?
            .trim()
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        Ok(Self {
            numbers,
            winning_numbers,
        })
    }
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    #[test]
    fn part_1() {
        let lines = read_lines("day4.txt").unwrap();
        let sum = part_one(&lines).unwrap();
        assert_eq!(sum, 25231);
    }

    #[test]
    fn part_2() {
        let lines = read_lines("day4.txt").unwrap();
        let sum = part_two(&lines).unwrap();
        assert_eq!(sum, 9721255);
    }
}

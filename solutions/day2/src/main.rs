use anyhow::{anyhow, Result};
use inputs::{read_lines, Lines};
use std::cmp;
use std::str::FromStr;

fn main() -> Result<()> {
    let lines = read_lines("day2.txt")?;

    println!("Day Two");
    println!("-------");
    println!("part one: {}", part_one(&lines)?);
    println!("part two: {}", part_two(&lines)?);

    Ok(())
}

fn part_one(lines: &Lines) -> Result<u32> {
    let sum = lines.iter().try_fold(0, |mut sum, line| {
        let game = Game::from_str(line)?;
        if game.is_possible() {
            sum += game.id;
        }
        Ok::<u32, anyhow::Error>(sum)
    })?;

    Ok(sum)
}

fn part_two(lines: &Lines) -> Result<u32> {
    let sum = lines.iter().try_fold(0, |mut sum, line| {
        let game = Game::from_str(line)?;
        sum += game.min_power();
        Ok::<u32, anyhow::Error>(sum)
    });

    Ok(sum?)
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.rounds
            .iter()
            .find(|round| round.red > 12 || round.green > 13 || round.blue > 14)
            .is_none()
    }

    fn min_power(&self) -> u32 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        self.rounds.iter().for_each(|round| {
            min_red = cmp::max(min_red, round.red);
            min_green = cmp::max(min_green, round.green);
            min_blue = cmp::max(min_blue, round.blue);
        });

        return min_red * min_green * min_blue;
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let id_part = parts.next().ok_or(anyhow!("no id part"))?;
        let rounds_part = parts.next().ok_or(anyhow!("no rounds part"))?;

        let id = id_part.trim_start_matches("Game ").parse()?;
        let rounds = rounds_part.split(';').map(|s| s.trim()).try_fold(
            Vec::new(),
            |mut acc, round_str| {
                acc.push(Round::from_str(round_str)?);
                Ok::<Vec<Round>, anyhow::Error>(acc)
            },
        )?;

        Ok(Game { id, rounds })
    }
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cube_str in s.split(',') {
            let mut cube_parts = cube_str.trim().split_whitespace();
            let count: u32 = cube_parts
                .next()
                .ok_or(anyhow!("no count part"))?
                .parse()
                .map_err(|_| anyhow!("invalid count"))?;

            let color = cube_parts.next().ok_or(anyhow!("no color part"))?;

            match color {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                _ => return Err(anyhow!("unknown color: {}", color)),
            }
        }

        Ok(Self { red, green, blue })
    }
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn part_1() {
        let lines = read_lines("day2.txt").unwrap();
        let sum = part_one(&lines).unwrap();
        assert_eq!(sum, 2810);
    }

    #[test]
    fn part_2() {
        let lines = read_lines("day2.txt").unwrap();
        let sum = part_two(&lines).unwrap();
        assert_eq!(sum, 69110);
    }
}

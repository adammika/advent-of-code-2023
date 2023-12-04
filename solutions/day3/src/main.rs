use anyhow::Result;
use inputs::{day_three_lines, Lines};
use std::str::FromStr;

fn main() -> Result<()> {
    let lines = day_three_lines()?;

    println!("Day Three");
    println!("-------");
    println!("part one: {}", part_one(&lines)?);
    println!("part two: {}", part_two(&lines)?);

    Ok(())
}

fn part_one(lines: &Lines) -> Result<u32> {
    let schematic = Schematic::from_lines(lines)?;

    let sum = schematic
        .numbers
        .iter()
        .filter(|n| {
            let x_range_start = if n.start_x > 0 { n.start_x - 1 } else { 0 };
            let target_x_range = x_range_start..=(n.end_x + 1);

            let y_range_start = if n.y > 0 { n.y - 1 } else { 0 };
            let target_y_range = y_range_start..=(n.y + 1);

            schematic
                .symbols
                .iter()
                .any(|s| target_x_range.contains(&s.x) && target_y_range.contains(&s.y))
        })
        .map(|n| n.value)
        .sum();

    Ok(sum)
}

fn part_two(lines: &Lines) -> Result<u32> {
    let schematic = Schematic::from_lines(lines)?;

    let sum = schematic
        .symbols
        .iter()
        .filter(|s| s.kind == '*')
        .map(|s| {
            let x_range_start = if s.x > 0 { s.x - 1 } else { 0 };
            let target_x_range = x_range_start..=(s.x + 1);

            let y_range_start = if s.y > 0 { s.y - 1 } else { 0 };
            let target_y_range = y_range_start..=(s.y + 1);

            let adjacent_numbers: Vec<u32> = schematic
                .numbers
                .iter()
                .filter(|n| {
                    (target_x_range.contains(&n.start_x) || target_x_range.contains(&n.end_x))
                        && target_y_range.contains(&n.y)
                })
                .map(|n| n.value)
                .collect();

            return if adjacent_numbers.len() == 2 {
                adjacent_numbers.iter().product()
            } else {
                0
            };
        })
        .sum();

    Ok(sum)
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
struct Number {
    value: u32,
    start_x: usize,
    end_x: usize,
    y: usize,
}

#[derive(Debug)]
struct Symbol {
    kind: char,
    x: usize,
    y: usize,
}

impl Schematic {
    fn from_lines(lines: &Lines) -> Result<Self> {
        let mut numbers: Vec<Number> = Vec::new();
        let mut symbols: Vec<Symbol> = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            let mut number = String::new();

            for (x, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    number.push(c);
                }

                if !c.is_numeric() && c != '.' {
                    symbols.push(Symbol { kind: c, x, y })
                }

                if !number.is_empty() && (!c.is_numeric() || x == line.len() - 1) {
                    numbers.push(Number {
                        value: number.parse()?,
                        start_x: x - number.len(),
                        end_x: x - 1,
                        y,
                    });
                    number = String::new();
                }
            }
        }

        Ok(Self { numbers, symbols })
    }
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    #[test]
    fn part_1() {
        let lines = day_three_lines().unwrap();
        let sum = part_one(&lines).unwrap();
        assert_eq!(sum, 550064);
    }

    #[test]
    fn part_2() {
        let lines = day_three_lines().unwrap();
        let sum = part_two(&lines).unwrap();
        assert_eq!(sum, 85010461);
    }
}

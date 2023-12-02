use anyhow::{anyhow, Result};
use inputs::{day_one_lines, Lines};

fn main() -> Result<()> {
    let lines = day_one_lines()?;

    println!("Day One");
    println!("-------");
    part_one(&lines)?;
    part_two(&lines)?;

    Ok(())
}

fn part_one(lines: &Lines) -> Result<()> {
    let mut sum: u32 = 0;

    for line in lines {
        let first = first_digit(line.chars())?;
        let last = first_digit(line.chars().rev())?;
        let combined = format!("{}{}", first, last);
        sum += combined.parse::<u32>().unwrap()
    }

    println!("part one: {}", sum);
    Ok(())
}

fn first_digit<I>(mut chars: I) -> Result<char>
where
    I: Iterator<Item = char>,
{
    chars
        .find(|c| c.is_numeric())
        .ok_or_else(|| anyhow!("no digit found"))
}

fn part_two(lines: &Lines) -> Result<()> {
    let mut sum: u32 = 0;

    for line in lines {
        let first = first_digit_maybe_from_word(line.chars())?;
        let last = first_digit_maybe_from_word(line.chars().rev())?;
        let combined = format!("{}{}", first, last);
        sum += combined.parse::<u32>().unwrap()
    }

    println!("part two: {}", sum);
    Ok(())
}

const DIGIT_WORDS: [([&str; 2], u32); 10] = [
    (["zero", "orez"], 0),
    (["one", "eno"], 1),
    (["two", "owt"], 2),
    (["three", "eerht"], 3),
    (["four", "ruof"], 4),
    (["five", "evif"], 5),
    (["six", "xis"], 6),
    (["seven", "neves"], 7),
    (["eight", "thgie"], 8),
    (["nine", "enin"], 9),
];

fn first_digit_maybe_from_word<I>(chars: I) -> Result<u32>
where
    I: Iterator<Item = char>,
{
    let mut s = String::new();
    for c in chars {
        if let Some(digit) = c.to_digit(10) {
            return Ok(digit);
        }

        s.push(c);

        for (words, digit) in DIGIT_WORDS.iter() {
            if s.contains(words[0]) || s.contains(words[1]) {
                return Ok(*digit);
            }
        }
    }

    Err(anyhow!("no digit found"))
}

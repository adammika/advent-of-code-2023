use anyhow::Result;
use inputs::{read_lines, Lines};

fn main() -> Result<()> {
    let lines = read_lines("day9.txt")?;

    println!("Day Nine");
    println!("-------");
    println!("part one: {}", part_one(&lines)?);
    println!("part two: {}", part_two(&lines)?);

    Ok(())
}

fn part_one(lines: &Lines) -> Result<isize> {
    extrapolated_sum(lines, true)
}

fn part_two(lines: &Lines) -> Result<isize> {
    extrapolated_sum(lines, false)
}

fn extrapolated_sum(lines: &Lines, forwards: bool) -> Result<isize> {
    Ok(lines
        .iter()
        .map(|l| l.split_whitespace().map(|c| c.parse::<isize>().unwrap()))
        .map(|nums| {
            let mut history: Vec<Vec<isize>> = vec![];
            history.push(nums.clone().collect());

            loop {
                let mut sequence: Vec<isize> = vec![];
                if let Some(last) = history.last() {
                    for w in last.windows(2) {
                        sequence.push(w[1] - w[0]);
                    }
                    history.push(sequence.clone());
                } else {
                    break;
                }

                if sequence.iter().all(|v| v == &0) {
                    break;
                }
            }

            history.iter().rev().fold(0, |acc, s| {
                if forwards {
                    acc + s[s.len() - 1]
                } else {
                    s[0] - acc
                }
            })
        })
        .sum())
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn part_1() {
        let lines = read_lines("day9.txt").unwrap();
        let answer = part_one(&lines).unwrap();
        assert_eq!(answer, 1882395907);
    }

    #[test]
    fn part_2() {
        let lines = read_lines("day9.txt").unwrap();
        let answer = part_two(&lines).unwrap();
        assert_eq!(answer, 1005);
    }
}

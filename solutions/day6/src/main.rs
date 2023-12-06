use anyhow::Result;
use inputs::{read_lines, Lines};

fn main() -> Result<()> {
    let lines = read_lines("day6.txt")?;
    println!("Lines: {:?}", lines);

    println!("Day Six");
    println!("-------");
    println!("part one: {}", part_one(&lines)?);
    println!("part two: {}", part_two(&lines)?);

    Ok(())
}

fn part_one(lines: &Lines) -> Result<usize> {
    let num_ways = parse_races(lines)
        .iter()
        .map(|&(time, target_distance)| {
            let mut num_ways_to_win_this_race = 0;
            for hold_time in 1..=time {
                if hold_time * (time - hold_time) >= target_distance {
                    num_ways_to_win_this_race += 1
                }
            }

            num_ways_to_win_this_race
        })
        .collect::<Vec<_>>();

    Ok(num_ways.iter().product())
}

fn part_two(lines: &Lines) -> Result<usize> {
    let (time, target_distance) = parse_race_bad_kerning(lines);

    let mut num_ways_to_win_this_race = 0;
    for hold_time in 1..=time {
        if hold_time * (time - hold_time) >= target_distance {
            num_ways_to_win_this_race += 1
        }
    }

    Ok(num_ways_to_win_this_race)
}

fn parse_races(lines: &Lines) -> Vec<(usize, usize)> {
    let times: Vec<usize> = lines[0]
        .trim_start_matches("Time:")
        .split_whitespace()
        .flat_map(|t| t.parse::<usize>())
        .collect();

    let distances: Vec<usize> = lines[1]
        .trim_start_matches("Distance:")
        .split_whitespace()
        .flat_map(|t| t.parse::<usize>())
        .collect();

    times.into_iter().zip(distances.into_iter()).collect()
}

fn parse_race_bad_kerning(lines: &Lines) -> (usize, usize) {
    let time: usize = lines[0]
        .trim_start_matches("Time:")
        .split_whitespace()
        .fold(String::new(), |acc, part| acc + part)
        .parse::<usize>()
        .unwrap();

    let distance: usize = lines[1]
        .trim_start_matches("Distance:")
        .split_whitespace()
        .fold(String::new(), |acc, part| acc + part)
        .parse::<usize>()
        .unwrap();

    (time, distance)
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn part_1() {
        let lines = read_lines("day6.txt").unwrap();
        let answer = part_one(&lines).unwrap();
        assert_eq!(answer, 4811940);
    }

    #[test]
    fn part_2() {
        let lines = read_lines("day6.txt").unwrap();
        let answer = part_two(&lines).unwrap();
        assert_eq!(answer, 30077773);
    }
}

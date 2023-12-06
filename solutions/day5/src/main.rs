use anyhow::Result;
use inputs::{read_lines, Lines};
use regex::Regex;
use std::ops::Range;
use std::usize;

fn main() -> Result<()> {
    let lines = read_lines("day5.txt")?;

    println!("Day Five");
    println!("-------");
    println!("part one: {}", part_one(&lines)?);
    println!("part two: {}", part_two(&lines)?);

    Ok(())
}

fn part_one(lines: &Lines) -> Result<usize> {
    let seeds_line = &lines[0];
    let seeds = seeds_line
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|c| c.parse::<usize>())
        .filter_map(Result::ok) // This will filter out Ok values and unwrap them
        .collect::<Vec<usize>>();

    let locations = parse_all_maps(lines).iter().fold(seeds, |seeds, map| {
        seeds
            .iter()
            .map(|s| {
                match map
                    .ranges
                    .iter()
                    .find(|r| r.source.contains(s))
                    .map(|r| r.dest.start + (s - r.source.start))
                {
                    Some(mapped) => mapped,
                    None => *s,
                }
            })
            .collect()
    });

    Ok(*locations.iter().min().unwrap())
}

fn part_two(lines: &Lines) -> Result<usize> {
    let seeds_line = &lines[0];
    let seeds = seeds_line
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|pair| {
            let start = pair[0];
            let len = pair[1];

            (start, start + len)
        })
        .collect::<Vec<_>>();

    let locations = parse_all_maps(lines).iter().fold(seeds, |seeds, map| {
        seeds
            .iter()
            .flat_map(|&(start, end)| {
                let mut mapped = Vec::new();
                let mut unmapped = vec![(start, end)];

                for range in &map.ranges {
                    let dst = range.dest.start;
                    let src = range.source.start;
                    let len = range.dest.end - range.dest.start;

                    let mut m = Vec::new();
                    for (start, end) in unmapped {
                        let a = (start, end.min(src));
                        let b = (start.max(src), (src + len).min(end));
                        let c = ((src + len).max(start), end);
                        if a.0 < a.1 {
                            m.push(a);
                        }
                        if b.0 < b.1 {
                            mapped.push((b.0 - src + dst, b.1 - src + dst));
                        }
                        if c.0 < c.1 {
                            m.push(c);
                        }
                    }
                    unmapped = m;
                }
                mapped
            })
            .collect()
    });

    let result = locations.iter().map(|r| r.0).min().unwrap();
    Ok(result)
}

fn parse_all_maps(lines: &Lines) -> Vec<Map> {
    let map_regex = Regex::new(r"^(?P<source>\w+)-to-(?P<destination>\w+) map:$").unwrap();
    let numbers_regex = Regex::new(r"^(\d+) (\d+) (\d+)$").unwrap();

    let mut all_maps: Vec<Map> = Vec::new();
    let mut curr_map = Map::default();

    for line in lines {
        if map_regex.is_match(&line) {
            curr_map = Map { ranges: vec![] };
        } else if numbers_regex.is_match(&line) {
            let numbers: Vec<usize> = line
                .split_whitespace()
                .filter_map(|n| n.parse::<usize>().ok())
                .collect();
            if numbers.len() == 3 {
                let d_start = numbers[0];
                let s_start = numbers[1];
                let len = numbers[2];

                curr_map.ranges.push(MapRange {
                    dest: (d_start..d_start + len).into(),
                    source: (s_start..s_start + len).into(),
                });
            }
        } else {
            if curr_map.ranges.iter().count() > 0 {
                all_maps.push(curr_map);
                curr_map = Map::default();
            }
        }
    }

    if curr_map.ranges.iter().count() > 0 {
        all_maps.push(curr_map);
    }

    all_maps
}

#[derive(Debug, Default)]
struct Map {
    ranges: Vec<MapRange>,
}

#[derive(Debug)]
struct MapRange {
    dest: Box<Range<usize>>,
    source: Box<Range<usize>>,
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn part_1() {
        let lines = read_lines("day5.txt").unwrap();
        let sum = part_one(&lines).unwrap();
        assert_eq!(sum, 313045984);
    }

    #[test]
    fn part_2() {
        let lines = read_lines("day5.txt").unwrap();
        let sum = part_two(&lines).unwrap();
        assert_eq!(sum, 20283860);
    }
}

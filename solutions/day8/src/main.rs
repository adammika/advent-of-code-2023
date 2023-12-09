use anyhow::Result;
use inputs::{read_lines, Lines};
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines = read_lines("day8.txt")?;

    println!("Day Eight");
    println!("-------");
    println!("part one: {}", part_one(&lines)?);
    println!("part two: {}", part_two(&lines)?);

    Ok(())
}

fn part_one(lines: &Lines) -> Result<usize> {
    let (directions, network) = parse(lines);

    let mut count = 0;
    let mut curr_node = "AAA";

    while curr_node != "ZZZ" {
        for char in &directions {
            let path = network[curr_node];
            if char == &'L' {
                curr_node = path.0;
            } else {
                curr_node = path.1;
            }
            count += 1;

            if curr_node == "ZZZ" {
                break;
            }
        }
    }

    Ok(count)
}

fn part_two(lines: &Lines) -> Result<usize> {
    let (directions, network) = parse(lines);

    let nodes: Vec<&str> = network
        .keys()
        .filter(|k| k.ends_with("A"))
        .cloned()
        .collect();

    let multiples = nodes
        .iter()
        .map(|n| {
            let mut first_z_encounter: Option<usize> = None;
            let mut count = 0;
            let mut iterate = true;
            let mut node = *n;

            while iterate {
                for c in &directions {
                    let (l, r) = network[node];
                    if c == &'L' {
                        node = l;
                    } else {
                        node = r;
                    }

                    count += 1;

                    if node.ends_with("Z") {
                        if let Some(_) = first_z_encounter {
                            iterate = false;
                        } else {
                            first_z_encounter = Some(count);
                        }
                    }
                }
            }

            return count - first_z_encounter.unwrap();
        })
        .collect::<Vec<_>>();

    Ok(lcm_of_vec(&multiples))
}

fn parse(lines: &Lines) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let mut iter = lines.iter();
    let directions = iter.next().unwrap().chars().collect();
    let _ = iter.next();

    let network = iter.fold(HashMap::new(), |mut acc, line| {
        let mut parts = line.split(" = ");
        let node = parts.next().unwrap();
        let path_path = parts.next().unwrap();
        let num_iter = &mut path_path[1..&path_path.len() - 1].split(", ");
        let l = num_iter.next().unwrap();
        let r = num_iter.next().unwrap();

        acc.insert(node, (l, r));
        acc
    });

    (directions, network)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn lcm_of_vec(numbers: &[usize]) -> usize {
    numbers.iter().copied().fold(1, |acc, num| lcm(acc, num))
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    #[test]
    fn part_1() {
        let lines = read_lines("day8.txt").unwrap();
        let answer = part_one(&lines).unwrap();
        assert_eq!(answer, 16897);
    }

    #[test]
    fn part_2() {
        let lines = read_lines("day8.txt").unwrap();
        let answer = part_two(&lines).unwrap();
        assert_eq!(answer, 16563603485021);
    }
}

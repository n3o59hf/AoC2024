use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;
use itertools::Itertools;
use std::fmt::Display;

// CodSpeed compatibility
#[allow(dead_code)]
pub fn part1(input: &str) -> impl Display {
    part1_solution(&parse(input))
}
#[allow(dead_code)]
pub fn part2(input: &str) -> impl Display {
    part2_solution(&parse(input))
}
// CodSpeed compatibility end

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<u64> {
    let input = input.trim_end();
    input
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn solve(input: &[u64], times: u8) -> usize {
    let mut data: FxHashMap<u64, usize> = FxHashMap::default();
    let counts = input.iter().counts_by(|x| x);
    for (x, c) in counts {
        data.insert(*x, c);
    }

    for _ in 0..times {
        let mut newdata = FxHashMap::default();
        for (k, size) in data {
            if k == 0 {
                let existing_size = newdata.get(&1).unwrap_or(&0);
                newdata.insert(1, size + existing_size);
            } else {
                let digits = k.ilog10() + 1;

                if digits % 2 == 0 {
                    let divisor = 10u64.pow(digits / 2);
                    let k1 = k / divisor;
                    let k2 = k % divisor;

                    let existing_size = newdata.get(&k1).unwrap_or(&0);
                    newdata.insert(k1, size + existing_size);

                    let existing_size = newdata.get(&k2).unwrap_or(&0);
                    newdata.insert(k2, size + existing_size);
                } else {
                    let new_k = k * 2024;
                    let existing_size = newdata.get(&new_k).unwrap_or(&0);
                    newdata.insert(new_k, size + existing_size);
                }
            }
        }
        data = newdata;
    }

    data.values().sum()
}

#[aoc(day11, part1)]
fn part1_solution(input: &[u64]) -> usize {
    solve(input, 25)
}

#[aoc(day11, part2)]
fn part2_solution(input: &[u64]) -> usize {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA: &str = "125 17";
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(TEST_DATA)), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(TEST_DATA)), 65601038650482);
    }
}

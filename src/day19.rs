use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

// CodSpeed compatibility
#[allow(dead_code, clippy::useless_format)]
pub fn part1(input: &str) -> String {
    format!("{}", part1_solution(&parse(input)))
}
#[allow(dead_code, clippy::useless_format)]
pub fn part2(input: &str) -> String {
    format!("{}", part2_solution(&parse(input)))
}
// CodSpeed compatibility end

type Input = (Vec<String>, Vec<String>);

#[aoc_generator(day19)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines();

    let patterns = lines
        .next()
        .expect("Patterns")
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let _ = lines.next();

    let towels = lines
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    (patterns, towels)
}

fn check_if_possible(
    towel: String,
    patterns: &Vec<String>,
    cache: &mut FxHashMap<String, bool>,
) -> bool {
    if towel.is_empty() {
        return true;
    }
    if let Some(result) = cache.get(&towel) {
        return *result;
    }

    for pattern in patterns {
        if towel.len() >= pattern.len() {
            let towel_part = &towel[0..pattern.len()];
            let pattern = pattern.as_str();
            if towel_part == pattern
                && check_if_possible(towel[pattern.len()..].to_string(), patterns, cache)
            {
                cache.insert(towel, true);
                return true;
            }
        }
    }
    cache.insert(towel, false);

    false
}
#[aoc(day19, part1)]
fn part1_solution(input: &Input) -> usize {
    input
        .1
        .par_iter()
        .filter(|towel| check_if_possible((*towel).clone(), &input.0, &mut FxHashMap::default()))
        .count()
}

#[aoc(day19, part1, Part2Solver)]
fn part1_solution_using_part2(input: &Input) -> usize {
    input
        .1
        .par_iter()
        .filter(|towel| {
            check_possible_ways((*towel).clone(), &input.0, &mut FxHashMap::default()) > 0
        })
        .count()
}
#[aoc(day19, part1, SharedCache)]
fn part1_solution_shared(input: &Input) -> usize {
    let mut cache = FxHashMap::default();
    input
        .1
        .iter()
        .filter(|towel| check_if_possible((*towel).clone(), &input.0, &mut cache))
        .count()
}

fn check_possible_ways(
    towel: String,
    patterns: &Vec<String>,
    cache: &mut FxHashMap<String, usize>,
) -> usize {
    if towel.is_empty() {
        return 1;
    }
    if let Some(result) = cache.get(&towel) {
        return *result;
    }

    for pattern in patterns {
        if towel.len() >= pattern.len() {
            let towel_part = &towel[0..pattern.len()];
            let pattern = pattern.as_str();
            if towel_part == pattern {
                let ways = check_possible_ways(towel[pattern.len()..].to_string(), patterns, cache);
                cache
                    .entry(towel.clone())
                    .and_modify(|e| *e += ways)
                    .or_insert(ways);
            }
        }
    }

    if let Some(result) = cache.get(&towel) {
        *result
    } else {
        0
    }
}

#[aoc(day19, part2)]
fn part2_solution(input: &Input) -> usize {
    input
        .1
        .par_iter()
        .map(|towel| check_possible_ways(towel.clone(), &input.0, &mut FxHashMap::default()))
        .sum()
}

#[aoc(day19, part2, SharedCache)]
fn part2_solution_shared(input: &Input) -> usize {
    let mut cache = FxHashMap::default();
    input
        .1
        .iter()
        .map(|towel| check_possible_ways(towel.clone(), &input.0, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(EXAMPLE)), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(EXAMPLE)), 16);
    }

    #[test]
    fn part1_example_part2() {
        assert_eq!(part1_solution_using_part2(&parse(EXAMPLE)), 6);
    }
    #[test]
    fn part1_example_shared() {
        assert_eq!(part1_solution_shared(&parse(EXAMPLE)), 6);
    }

    #[test]
    fn part2_example_shared() {
        assert_eq!(part2_solution_shared(&parse(EXAMPLE)), 16);
    }
}

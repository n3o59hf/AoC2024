use aoc_runner_derive::{aoc, aoc_generator};
use prse::parse;
use std::fmt::Display;
use std::slice::Iter;

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

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let (result, parts): (u64, &str) = parse!(l, "{}: {}");
            let parts = parts
                .split(' ')
                .map(|x| x.parse::<u64>())
                .filter_map(Result::ok)
                .collect();
            (result, parts)
        })
        .collect()
}

fn concat(a: u64, b: u64) -> u64 {
    let digits = if b == 0 {
        1
    } else {
        (b as f64).log10().floor() as u32 + 1
    };

    a * 10u64.pow(digits) + b
}

fn check_expression(result: u64, first: u64, rest: Iter<u64>, allow_concat: bool) -> bool {
    if first > result {
        return false;
    }
    let mut rest = rest.clone();
    let second = rest.next();
    match second {
        None => result == first,
        Some(second) => {
            check_expression(result, first + second, rest.clone(), allow_concat)
                || check_expression(result, first * second, rest.clone(), allow_concat)
                || (allow_concat
                    && check_expression(result, concat(first, *second), rest.clone(), allow_concat))
        }
    }
}

#[aoc(day7, part1)]
fn part1_solution(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter(|(r, d)| {
            let mut iter = d.iter();
            let first = iter.next().expect("Should be at least one item");
            check_expression(*r, *first, iter, false)
        })
        .map(|(r, _)| r)
        .sum()
}

#[aoc(day7, part2)]
fn part2_solution(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .iter()
        .filter(|(r, d)| {
            let mut iter = d.iter();
            let first = iter.next().expect("Should be at least one item");
            check_expression(*r, *first, iter, true)
        })
        .map(|(r, _)| r)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(TEST_DATA)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(TEST_DATA)), 11387);
    }
}

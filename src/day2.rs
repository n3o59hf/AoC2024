use aoc_runner_derive::{aoc, aoc_generator};
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

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut data: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let reports: Vec<i32> = line
            .split_whitespace()
            .map(|i| i.parse::<i32>().unwrap())
            .collect();

        data.push(reports);
    }
    data
}

fn is_safe(reports: &[i32]) -> bool {
    if reports.len() < 2 {
        false
    } else {
        let deltas: Vec<i32> = reports.windows(2).map(|p| p[1] - p[0]).collect();

        let all_positive = deltas.iter().all(|&d| d > 0);
        let all_negative = deltas.iter().all(|&d| d < 0);
        let all_nearby = deltas.iter().all(|&d| d.abs() <= 3);

        (all_positive || all_negative) && all_nearby
    }
}
#[aoc(day2, part1)]
fn part1_solution(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|reports| if is_safe(reports) { 1 } else { 0 })
        .sum()
}

#[aoc(day2, part2)]
fn part2_solution(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|reports| {
            if is_safe(reports) {
                1
            } else {
                let mut variants = (0..reports.len()).map(|i| {
                    reports
                        .iter()
                        .enumerate()
                        .filter(move |(j, _)| i != *j)
                        .map(|(_, p)| *p)
                });

                if variants.any(|variant| is_safe(&variant.collect::<Vec<i32>>())) {
                    1
                } else {
                    0
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(TEST_DATA)), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(TEST_DATA)), 4);
    }
}

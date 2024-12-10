use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::fmt::{Debug, Display, Formatter};

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

enum Instructions {
    Mul(i32, i32),
    Do,
    Dont,
}

impl Debug for Instructions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::Mul(n1, n2) => write!(f, "mul({}, {})", n1, n2),
            Instructions::Do => write!(f, "do()"),
            Instructions::Dont => write!(f, "don't()"),
        }
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Instructions> {
    let mut output: Vec<Instructions> = Vec::new();
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").expect("Invalid regex");

    for cap in regex.captures_iter(input) {
        if let Some(mul_match) = cap.get(0) {
            let mul_match = mul_match.as_str();
            if mul_match.starts_with("mul(") {
                if let (Ok(n1), Ok(n2)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                    output.push(Instructions::Mul(n1, n2));
                }
            } else if mul_match == "do()" {
                output.push(Instructions::Do);
            } else if mul_match == "don't()" {
                output.push(Instructions::Dont);
            }
        }
    }
    output
}

#[aoc(day3, part1)]
fn part1_solution(input: &[Instructions]) -> i32 {
    input
        .iter()
        .map(|x| {
            if let Instructions::Mul(n1, n2) = x {
                n1 * n2
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2_solution(input: &[Instructions]) -> i32 {
    let mut sum = 0;
    let mut enabled = true;
    for instruction in input {
        match instruction {
            Instructions::Mul(n1, n2) => {
                if enabled {
                    sum += n1 * n2;
                }
            }
            Instructions::Do => enabled = true,
            Instructions::Dont => enabled = false,
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1_solution(&parse(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )),
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2_solution(&parse(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            48
        );
    }
}

use aoc_runner_derive::{aoc, aoc_generator};
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
#[derive(Clone, Copy)]
struct ClawMachine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

impl ClawMachine {
    pub fn parse(line_a: &str, line_b: &str, line_p: &str) -> Self {
        let (ax, ay) = Self::parse_line(line_a, 12);
        let (bx, by) = Self::parse_line(line_b, 12);
        let (px, py) = Self::parse_line(line_p, 9);
        Self {
            ax,
            ay,
            bx,
            by,
            px,
            py,
        }
    }
    #[inline]
    fn parse_line(line: &str, x_start: usize) -> (i64, i64) {
        let separator = line.rfind(',').expect("Bad input");
        let y_start = separator + 4;
        let x: i64 = line[x_start..separator].parse().expect("Bad input");
        let y: i64 = line[y_start..].parse().expect("Bad input");
        (x, y)
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<ClawMachine> {
    let mut i = 0;
    input
        .lines()
        .filter(|_| {
            let keep = i % 4 != 3;
            i += 1;
            keep
        })
        .tuples::<(&str, &str, &str)>()
        .map(|(a, b, p)| ClawMachine::parse(a, b, p))
        .collect()
}

fn solve(input: &[ClawMachine], offset: i64) -> i64 {
    input
        .iter()
        .map(|c| {
            let ax = c.ax;
            let ay = c.ay;
            let bx = c.bx;
            let by = c.by;
            let px = c.px + offset;
            let py = c.py + offset;
            let b = (px * ay - py * ax) / (bx * ay - ax * by);
            let bbx = b * bx;
            let a = (px - bbx) / ax;
            if px == a * ax + bbx && py == a * ay + b * by {
                a * 3 + b
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day13, part1)]
fn part1_solution(input: &[ClawMachine]) -> i64 {
    solve(input, 0)
}

#[aoc(day13, part2)]
fn part2_solution(input: &[ClawMachine]) -> i64 {
    solve(input, 10000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA: &str = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(TEST_DATA)), 480);
    }
}

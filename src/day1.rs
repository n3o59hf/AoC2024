use aoc_runner_derive::{aoc, aoc_generator};
use prse::parse;
use std::fmt::Display;
// CodSpeed compatibility
#[allow(dead_code)]
pub fn part1(input: &str) -> impl Display {
    part1_solution(&parse(input))
}
#[allow(dead_code)]
fn part2(input: &str) -> impl Display {
    part2_solution(&parse(input))
}
// CodSpeed compatibility end

#[aoc_generator(day1)]
fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines();
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in lines {
        let (n1, n2): (i32, i32) = parse!(line, "{} {}");
        list1.push(n1);
        list2.push(n2);
    }
    (list1, list2)
}

#[aoc(day1, part1)]
fn part1_solution(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (mut list1, mut list2) = input.clone();

    list1.sort();
    list2.sort();
    list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>()
}

#[aoc(day1, part2)]
fn part2_solution(input: &(Vec<i32>, Vec<i32>)) -> i32 {
    let (mut list1, mut list2) = input.clone();

    list1.sort();
    list2.sort();
    let mut sim_score = 0;

    for x in list1 {
        let score = list2.iter().filter(|y| x == **y).count() as i32;
        sim_score += score * x;
    }

    sim_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(INPUT)), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(INPUT)), 31);
    }
}

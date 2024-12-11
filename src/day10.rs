use crate::utils::c2::{C2Field, C2};
use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;
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
#[aoc_generator(day10)]
fn parse(input: &str) -> C2Field<usize> {
    let input = input.trim_end();
    C2Field::from_string(input, |h| {
        h.to_digit(10).expect("Should be a digit") as usize
    })
}

#[inline(always)]
fn traverse_part_1(from: &C2, map: &C2Field<usize>) -> Vec<C2> {
    if let Some(h) = map.get(from) {
        if *h == 9 {
            vec![*from]
        } else {
            from.neighbors_4()
                .iter()
                .filter(|c| map.get(c) == Some(&(h + 1)))
                .flat_map(|c| traverse_part_1(c, map))
                .collect()
        }
    } else {
        vec![]
    }
}

#[aoc(day10, part1)]
fn part1_solution(input: &C2Field<usize>) -> usize {
    let starts: Vec<&C2> = input
        .iter()
        .filter(|(_, &h)| h == 0)
        .map(|(k, _)| k)
        .collect();

    starts
        .par_iter()
        .map(|c| traverse_part_1(c, input))
        .flat_map(|t| t.iter().cloned().collect::<HashSet<_>>())
        .count()
}

#[inline(always)]
fn traverse_part_2(from: &C2, map: &C2Field<usize>) -> usize {
    if let Some(h) = map.get(from) {
        if *h == 9 {
            1
        } else {
            from.neighbors_4()
                .iter()
                .filter(|c| map.get(c) == Some(&(h + 1)))
                .map(|c| traverse_part_2(c, map))
                .sum()
        }
    } else {
        0
    }
}

#[aoc(day10, part2)]
fn part2_solution(input: &C2Field<usize>) -> usize {
    let starts: Vec<&C2> = input
        .iter()
        .filter(|(_, &h)| h == 0)
        .map(|(k, _)| k)
        .collect();

    starts.par_iter().map(|c| traverse_part_2(c, input)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(EXAMPLE_DATA)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(EXAMPLE_DATA)), 81);
    }
}

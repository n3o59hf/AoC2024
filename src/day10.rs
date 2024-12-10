use rayon::iter::ParallelIterator;
use crate::utils::c2::{C2, C2_4_NEIGHBORS};
use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;
use std::collections::HashSet;
use std::fmt::Display;
use rayon::iter::IntoParallelRefIterator;

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
fn parse(input: &str) -> FxHashMap<C2, usize> {
    let mut map: FxHashMap<_, _> = FxHashMap::default();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let h = c.to_digit(10).unwrap();
            map.insert(C2::new(x as i32, y as i32), h as usize);
        }
    }

    map
}

#[inline(always)]
fn traverse_part_1(from: &C2, map: &FxHashMap<C2, usize>) -> Vec<C2> {
    if let Some(h) = map.get(from) {
        if *h == 9 {
            vec![*from]
        } else {
            C2_4_NEIGHBORS
                .iter()
                .map(|c| (*c) + (*from))
                .filter(|c| map.get(c) == Some(&(h + 1)))
                .flat_map(|c| traverse_part_1(&c, map))
                .collect()
        }
    } else {
        vec![]
    }
}

#[aoc(day10, part1)]
fn part1_solution(input: &FxHashMap<C2, usize>) -> usize {
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
fn traverse_part_2(from: &C2, map: &FxHashMap<C2, usize>) -> usize {
    if let Some(h) = map.get(from) {
        if *h == 9 {
            1
        } else {
            C2_4_NEIGHBORS
                .iter()
                .map(|c| (*c) + (*from))
                .filter(|c| map.get(c) == Some(&(h + 1)))
                .map(|c| traverse_part_2(&c, map))
                .sum()
        }
    } else {
        0
    }
}

#[aoc(day10, part2)]
fn part2_solution(input: &FxHashMap<C2, usize>) -> usize {
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

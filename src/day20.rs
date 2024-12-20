use crate::utils::c2::{C2Field, C2};
use crate::utils::maze::{parse_maze, Maze};
use aoc_runner_derive::{aoc, aoc_generator};
use binary_heap_plus::BinaryHeap;
use fxhash::FxHashMap;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::io::Write;

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

#[aoc_generator(day20)]
fn parse(input: &str) -> Maze {
    parse_maze(input)
}

fn route(field: &C2Field<bool>, start: C2, end: C2) -> Vec<C2> {
    let mut moves_to_check = BinaryHeap::new_by_key(|(c, _)| *c);
    moves_to_check.push((0, start));
    let mut prices: FxHashMap<C2, u32> = FxHashMap::default();
    prices.insert(start, 0);
    while let Some((p, m)) = moves_to_check.pop() {
        std::io::stdout().flush().unwrap();
        if m == end {
            break;
        }
        if let Some(price) = prices.get(&m) {
            if *price < p {
                continue;
            }
        }
        let price = p + 1;
        for next in m.neighbors_4() {
            if *field.get(&next).unwrap_or(&false) {
                if let Some(next_price) = prices.get(&next) {
                    if *next_price < price {
                        continue;
                    }
                }
                moves_to_check.push((price, next));
                prices.insert(next, price);
            }
        }
    }

    let mut backtrace: Vec<C2> = vec![end];
    while backtrace.last() != Some(&start) {
        let candidates = backtrace.last().expect("Should be here").neighbors_4();
        let next = candidates
            .iter()
            .min_by_key(|c| prices.get(c).unwrap_or(&u32::MAX))
            .expect("Should be here");
        backtrace.push(*next);
    }
    backtrace.reverse();
    backtrace
}

fn solve_for_constraints(input: &Maze, cheat_length: usize, cheat_cutoff: usize) -> usize {
    let cheat_length = cheat_length as u32;
    let base_vec = route(&input.0, input.1, input.2);
    let base = base_vec.as_slice();
    let base_len = base.len();

    base[..base_len - cheat_cutoff]
        .par_iter()
        .enumerate()
        .map(|(a, &ca)| {
            let mut count = 0;
            for (b, &cb) in base[a + cheat_cutoff..].iter().enumerate() {
                let d = (ca - cb).to_manhattan();
                let save = b + cheat_cutoff - d as usize;
                if d <= cheat_length && save >= cheat_cutoff {
                    count += 1;
                }
            }
            count
        })
        .sum()
}

#[aoc(day20, part1)]
fn part1_solution(input: &Maze) -> usize {
    solve_for_constraints(input, 2, 100)
}

#[aoc(day20, part2)]
fn part2_solution(input: &Maze) -> usize {
    solve_for_constraints(input, 20, 100)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
    #[test]
    fn part1_example() {
        assert_eq!(solve_for_constraints(&parse(EXAMPLE), 2, 20), 5);
    }

    #[test]
    fn part1_example_full() {
        assert_eq!(solve_for_constraints(&parse(EXAMPLE), 2, 2), 44);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_for_constraints(&parse(EXAMPLE), 20, 50), 285);
    }
}

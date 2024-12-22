use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;
use itertools::Itertools;
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

trait SecretIterable {
    fn next_secret(self) -> Self;

    fn iterator(self) -> impl Iterator<Item = Self>;
}

impl SecretIterable for u64 {
    #[inline]
    fn next_secret(self) -> Self {
        let mut input = self;
        input ^= (input << 6) & 0xFFFFFF;
        input ^= input >> 5;
        input ^ (input << 11) & 0xFFFFFF
    }

    fn iterator(self) -> impl Iterator<Item = Self> {
        let mut current = self;

        std::iter::from_fn(move || {
            let ret = current;
            current = current.next_secret();
            Some(ret)
        })
        .take(2001)
    }
}
#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter_map(|l| l.trim().parse::<u64>().ok())
        .collect()
}

fn get_sell_sequences(input: impl Iterator<Item = u64>) -> FxHashMap<(i8, i8, i8, i8), u64> {
    let mut output = FxHashMap::default();
    let input_vec = input.collect_vec();
    let window = input_vec.windows(5);
    for w in window {
        let (a, b, c, d, e) = (
            (w[0] % 10) as i8,
            (w[1] % 10) as i8,
            (w[2] % 10) as i8,
            (w[3] % 10) as i8,
            (w[4] % 10) as i8,
        );
        let (da, db, dc, dd) = (b - a, c - b, d - c, e - d);
        output.entry((da, db, dc, dd)).or_insert(e as u64);
    }

    output
}

#[aoc(day22, part1)]
fn part1_solution(input: &[u64]) -> u64 {
    input
        .par_iter()
        .map(|i| i.iterator().nth(2000).expect("Secret number"))
        .sum()
}

#[aoc(day22, part2)]
fn part2_solution(input: &[u64]) -> u64 {
    *input
        .par_iter()
        .map(|&i| get_sell_sequences(i.iterator()))
        .reduce(FxHashMap::default, |mut acc, map| {
            for (key, value) in map {
                *acc.entry(key).or_insert(0) += value;
            }
            acc
        })
        .values()
        .max()
        .expect("No max value found!")
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"1
10
100
2024"#;

    const EXAMPLE2: &str = r#"1
2
3
2024"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(EXAMPLE)), 37327623);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(EXAMPLE2)), 23);
    }
}

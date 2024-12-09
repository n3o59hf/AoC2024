use crate::utils::c2::{C2, C2_8_NEIGHBORS};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
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

struct WordField {
    letters: HashMap<C2, char>,
}

static TOP_LEFT: C2 = C2::new(-1, -1);
static TOP_RIGHT: C2 = C2::new(1, -1);
static BOTTOM_LEFT: C2 = C2::new(-1, 1);
static BOTTOM_RIGHT: C2 = C2::new(1, 1);

static XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

impl WordField {
    fn parse(input: String) -> Self {
        let mut letters = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                letters.insert(C2::new(x as i32, y as i32), c);
            }
        }
        Self { letters }
    }

    fn count_xmas(&self, c: &C2) -> i32 {
        if let Some(char) = self.letters.get(c) {
            if *char != XMAS[0] {
                return 0;
            }
        } else {
            return 0;
        }

        let mut count = 0;
        'words: for d in C2_8_NEIGHBORS {
            for (k, l) in XMAS.iter().enumerate().skip(1) {
                if let Some(char) = self.letters.get(&(*c + (d * k as i32))) {
                    if char != l {
                        continue 'words;
                    }
                } else {
                    continue 'words;
                }
            }
            count += 1;
        }
        count
    }

    fn is_mas(&self, coord: &C2) -> bool {
        if let Some(c) = self.letters.get(coord) {
            if *c == 'A' {
                // Pairs a-c, b-d
                if let Some(a) = self.letters.get(&(*coord + TOP_LEFT)) {
                    if let Some(b) = self.letters.get(&(*coord + TOP_RIGHT)) {
                        if let Some(c) = self.letters.get(&(*coord + BOTTOM_RIGHT)) {
                            if let Some(d) = self.letters.get(&(*coord + BOTTOM_LEFT)) {
                                if ((*a == 'M' && *c == 'S') || (*a == 'S' && *c == 'M'))
                                    && ((*b == 'M' && *d == 'S') || (*b == 'S' && *d == 'M'))
                                {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }

    fn get_all_coordinates(&self) -> Vec<C2> {
        self.letters.keys().cloned().collect()
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> WordField {
    WordField::parse(input.to_string())
}

#[aoc(day4, part1)]
fn part1_solution(input: &WordField) -> i32 {
    input
        .get_all_coordinates()
        .iter()
        .map(|c| input.count_xmas(c))
        .sum()
}

#[aoc(day4, part2)]
fn part2_solution(input: &WordField) -> i32 {
    input
        .get_all_coordinates()
        .iter()
        .filter(|c| input.is_mas(c))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(TEST_INPUT)), 18);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(TEST_INPUT)), 9);
    }
}

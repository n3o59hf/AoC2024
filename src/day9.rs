use aoc_runner_derive::{aoc, aoc_generator};
use std::clone::Clone;
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Entry {
    Taken(File),
    Free(u32),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct File {
    id: u32,
    length: u32,
}

impl File {
    fn new(id: u32, length: u32) -> Self {
        Self { id, length }
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Entry> {
    let input = input.trim_end();
    let mut id = 0u32;
    let mut empty = false;
    let mut output = Vec::new();
    for c in input.chars() {
        let d = c.to_digit(10).expect("Bad input");
        if !empty {
            if d > 0 {
                output.push(Entry::Taken(File::new(id, d)));
            }
            id += 1;
        } else if d > 0 {
            output.push(Entry::Free(d));
        }

        empty = !empty;
    }
    output
}

fn expand(input: &Vec<Entry>) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::new();
    for x in input {
        match x {
            Entry::Taken(file) => {
                for _ in 0..file.length {
                    output.push(file.id as i64);
                }
            }
            Entry::Free(space) => {
                for _ in 0..*space {
                    output.push(-1);
                }
            }
        }
    }

    output
}

fn checksum(data: &[i64]) -> u64 {
    data.iter()
        .enumerate()
        .filter(|(_, x)| **x >= 0)
        .map(|(i, x)| (i as u64) * (*x as u64))
        .sum()
}

#[aoc(day9, part1)]
fn part1_solution(input: &Vec<Entry>) -> u64 {
    let mut data = expand(input);

    let mut a = 0usize;
    let mut b = data.len() - 1;

    while a < b {
        while data[a] >= 0 {
            a += 1
        }

        while data[b] < 0 {
            b -= 1
        }

        if a > b {
            break;
        };

        data[a] = data[b];
        data[b] = -1;
        a += 1;
        b -= 1;
    }

    checksum(&data)
}

fn merge_free_spaces(vec: Vec<Entry>) -> Vec<Entry> {
    let mut result = Vec::new();

    for item in vec {
        match (result.last_mut(), item) {
            (Some(Entry::Free(existing)), Entry::Free(new)) => {
                *existing += new;
            }
            (_, new_item) => result.push(new_item),
        }
    }

    result
}

#[aoc(day9, part2)]
fn part2_solution(input: &[Entry]) -> u64 {
    let mut data = input.to_owned();
    data = merge_free_spaces(data);
    let mut to_process: Vec<&File> = input
        .iter()
        .filter_map(|x| match x {
            Entry::Taken(f) => Some(f),
            Entry::Free(_) => None,
        })
        .collect();

    to_process.sort_by(|a, b| a.id.cmp(&b.id));
    to_process.reverse();

    for f in to_process {
        let file_index = data
            .iter()
            .position(|x| *x == Entry::Taken(*f))
            .expect("Should be present");

        let empty_index = data.iter().position(|x| match x {
            Entry::Taken(_) => false,
            Entry::Free(size) => *size >= f.length,
        });

        if let Some(empty_index) = empty_index {
            if empty_index < file_index {
                let file = data.remove(file_index);
                let empty = data.remove(empty_index);
                data.insert(empty_index, file);
                data.insert(file_index, Entry::Free(f.length));
                if let Entry::Free(size) = empty {
                    if size > f.length {
                        data.insert(empty_index + 1, Entry::Free(size - f.length));
                    }
                } else {
                    panic!("Should be free");
                }
                data = merge_free_spaces(data);
            }
        }
    }
    checksum(&expand(&data))
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA_SIMPLE: &str = "12345";
    const TEST_DATA: &str = "2333133121414131402";

    #[test]
    fn part1_example_simple() {
        assert_eq!(part1_solution(&parse(TEST_DATA_SIMPLE)), 60);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(TEST_DATA)), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(TEST_DATA)), 2858);
    }
}

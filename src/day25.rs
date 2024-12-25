use aoc_runner_derive::{aoc, aoc_generator};
// CodSpeed compatibility
#[allow(dead_code, clippy::useless_format)]
pub fn part1(input: &str) -> String {
    format!("{}", part1_solution(&parse(input)))
}
#[allow(dead_code, clippy::useless_format)]
pub fn part2(_: &str) -> String {
    "".to_owned()
}
// CodSpeed compatibility end

type Input = (Vec<u32>, Vec<u32>);

const LOCKS: [u32; 6] = [0b00000, 0b00001, 0b00011, 0b00111, 0b01111, 0b11111];

const KEYS: [u32; 6] = [0b00000, 0b10000, 0b11000, 0b11100, 0b11110, 0b11111];

#[aoc_generator(day25)]
fn parse(input: &str) -> Input {
    let mut locks: Vec<u32> = Vec::new();
    let mut keys: Vec<u32> = Vec::new();
    let mut number: [usize; 5] = [0, 0, 0, 0, 0];

    let bytes = input.as_bytes();
    let stop_point = bytes.len() - 8;
    let mut cursor = 0usize;
    while cursor < stop_point {
        number.fill(0);
        let is_key = bytes[cursor] == b'.';
        cursor += 6; // skip first line
        for _ in 0..5 {
            for i in 0..5 {
                if bytes[cursor + i] == b'#' {
                    number[i] += 1;
                }
            }
            cursor += 6;
        }
        cursor += 7; // skip last line and empty line
        let map = if is_key { KEYS } else { LOCKS };
        let mut pins = 0u32;
        for i in number {
            pins <<= 5;
            pins |= map[i] << (5 * i);
        }

        if is_key { &mut keys } else { &mut locks }.push(pins);
    }
    (keys, locks)
}

#[aoc(day25, part1)]
fn part1_solution(input: &Input) -> u32 {
    let mut counter = 0u32;
    for &a in &input.0 {
        for &b in &input.1 {
            if a & b == 0 {
                counter += 1;
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(EXAMPLE)), 3);
    }
}

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

#[aoc_generator(day25)]
fn parse(input: &str) -> Input {
    let mut locks: Vec<u32> = Vec::with_capacity(250);
    let mut keys: Vec<u32> = Vec::with_capacity(250);
    let mut number: [u8; 5] = [0, 0, 0, 0, 0];
    for c in input.split("\n\n") {
        number.fill(0);
        for line in c.lines().skip(1).take(5) {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    number[4 - i] += 1;
                }
            }
        }
        if &c[0..1] == "." {
            //key
            let mut pins = 0u32;
            for i in number {
                for _ in 0..i {
                    pins <<= 1;
                    pins += 1;
                }
                for _ in 0..(5 - i) {
                    pins <<= 1;
                }
            }
            keys.push(pins);
        } else {
            //lock
            let mut pins = 0u32;
            for i in number {
                for _ in 0..(5 - i) {
                    pins <<= 1;
                }
                for _ in 0..i {
                    pins <<= 1;
                    pins += 1;
                }
            }
            locks.push(pins);
        }
    }
    (keys, locks)
}

#[aoc(day25, part1)]
fn part1_solution(input: &Input) -> u32 {
    let (keys, locks) = input.clone();
    let mut counter = 0u32;
    for lock in locks {
        for &key in &keys {
            if key & lock == 0 {
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

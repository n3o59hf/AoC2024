use crate::utils::c2::{C2Field, C2, C2_DOWN, C2_LEFT, C2_RIGHT, C2_UP};
use aoc_runner_derive::{aoc, aoc_generator};
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

#[aoc_generator(day12)]
fn parse(input: &str) -> C2Field<char> {
    let input = input.trim();
    C2Field::from_string(input, |c| c)
}

fn calculate_perimeter_and_area(
    map: &C2Field<char>,
    visited: &mut Vec<bool>,
    position: &C2,
    values: (u32, u32),
) -> (u32, u32) {
    let mut neighbors = 0;
    let mut values = values;
    let letter = *map
        .get(position)
        .expect("Should not be called on non existing position");
    visited[map.indice(position)] = true;
    for next in position.neighbors_4() {
        if map.get(&next) == Some(&letter) {
            if !visited[map.indice(&next)] {
                values = calculate_perimeter_and_area(map, visited, &next, values);
            }
            neighbors += 1;
        }
    }
    (values.0 + (4 - neighbors), values.1 + 1)
}

#[aoc(day12, part1)]
fn part1_solution(input: &C2Field<char>) -> u32 {
    let mut visited = input.values().iter().map(|_| false).collect::<Vec<_>>();
    let mut fence = 0;
    let keys = input.keys().clone();
    for c in keys.iter() {
        if !visited[input.indice(c)] {
            let (perimeter, area) = calculate_perimeter_and_area(input, &mut visited, c, (0, 0));
            fence += perimeter * area;
        }
    }
    fence
}

const UP: u8 = 0b0001;
const DOWN: u8 = 0b0010;
const LEFT: u8 = 0b0100;
const RIGHT: u8 = 0b1000;

const DIRECTIONS: [(C2, u8); 4] = [
    (C2_UP, UP),
    (C2_LEFT, LEFT),
    (C2_DOWN, DOWN),
    (C2_RIGHT, RIGHT),
];
const VISITED: u8 = 0b10000;
fn calculate_sides(map: &C2Field<char>, perimeter: &mut Vec<u8>, position: &C2) -> u32 {
    let letter = *map
        .get(position)
        .expect("Should not be called on non existing position");
    perimeter[map.indice(position)] = VISITED;

    let mut count = 1;

    for (cd, d) in DIRECTIONS.iter() {
        let next = *position + *cd;
        if map.get(&next) != Some(&letter) {
            perimeter[map.indice(position)] |= d;
        } else if perimeter[map.indice(&next)] == 0 {
            let sides = calculate_sides(map, perimeter, &next);
            count += sides;
        }
    }
    count
}

fn count_sides(sides: &[u8], width: usize) -> u32 {
    let height = sides.len() / width;
    let mut count = 0;

    // horizontal
    for y in 0..height {
        let mut up: bool = false;
        let mut down: bool = false;

        for x in 0..width {
            let c = y * width + x;
            if sides[c] & UP != 0 {
                if !up {
                    up = true;
                    count += 1;
                }
            } else {
                up = false;
            }
            if sides[c] & DOWN != 0 {
                if !down {
                    down = true;
                    count += 1;
                }
            } else {
                down = false;
            }
        }
    }

    // vertical
    for x in 0..width {
        let mut left: bool = false;
        let mut right: bool = false;

        for y in 0..height {
            let c = y * width + x;
            if sides[c] & LEFT != 0 {
                if !left {
                    left = true;
                    count += 1;
                }
            } else {
                left = false;
            }

            if sides[c] & RIGHT != 0 {
                if !right {
                    right = true;
                    count += 1;
                }
            } else {
                right = false;
            }
        }
    }

    count
}

#[aoc(day12, part2)]
fn part2_solution(input: &C2Field<char>) -> u32 {
    let mut sides = input.values().iter().map(|_| 0u8).collect::<Vec<_>>();
    let mut visited = input.values().iter().map(|_| false).collect::<Vec<_>>();
    let mut fence = 0;
    let keys = input.keys().clone();
    for c in keys.iter() {
        if !visited[input.indice(c)] {
            sides.fill(0);
            let count = calculate_sides(input, &mut sides, c);

            let sides_count = count_sides(&sides, input.width());

            fence += count * sides_count;
            for (i, _) in sides.iter().enumerate().filter(|&(_, s)| *s != 0) {
                visited[i] = true;
            }
        }
    }
    fence
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;

    const TEST_DATA_E: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;
    const TEST_DATA_HOLES: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(TEST_DATA)), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(TEST_DATA)), 1206);
    }

    #[test]
    fn part2_e() {
        assert_eq!(part2_solution(&parse(TEST_DATA_E)), 236);
    }
    #[test]
    fn part2_holes() {
        assert_eq!(part2_solution(&parse(TEST_DATA_HOLES)), 368);
    }
}

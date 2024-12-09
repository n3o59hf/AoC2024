use crate::utils::c2::C2;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};
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

fn group_antennas(input: Vec<(char, C2)>) -> HashMap<char, Vec<C2>> {
    let mut groups: HashMap<char, Vec<C2>> = HashMap::new();
    for (freq, location) in input {
        groups.entry(freq).or_default().push(location);
    }
    groups
}
struct Field {
    antennas: HashMap<char, Vec<C2>>,
    border: C2,
}
#[aoc_generator(day8)]
fn parse(input: &str) -> Field {
    let lines = input.lines();
    let max_y = lines.count();
    let mut max_x = 0;
    let mut antennas: Vec<(char,C2)> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            max_x = line.chars().count();
        }
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.push((c, C2::new(x as i32, y as i32)));
            }
        }
    }

    Field {
        antennas: group_antennas(antennas),
        border: C2::new(max_x as i32, max_y as i32),
    }
}


#[aoc(day8, part1)]
fn part1_solution(input: &Field) -> usize {
    let mut antinodes: HashSet<C2> = HashSet::new();

    for (_, locations) in input.antennas.iter() {
        for a in locations {
            for b in locations {
                if a != b {
                    antinodes.insert((*a).mirror(*b));
                }
            }
        }
    }

    antinodes
        .iter()
        .filter(|n| n.x >= 0 && n.y >= 0 && n.x < input.border.x && n.y < input.border.y)
        .count()
}

#[aoc(day8, part2)]
fn part2_solution(input: &Field) -> usize {
    let mut antinodes: HashSet<C2> = HashSet::new();

    for (_, locations) in input.antennas.iter() {
        for a in locations {
            for b in locations {
                if a != b {
                    let a = *a;
                    let b = *b;
                    let delta = a - b;
                    
                    let mut current = a;
                    while current.x >= 0 && current.y >= 0 && current.x < input.border.x && current.y < input.border.y {
                        antinodes.insert(current); 
                        current = current + delta;
                    }
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(TEST_DATA)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(TEST_DATA)),34);
    }
}

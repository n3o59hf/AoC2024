use crate::utils::c2::C2;
use aoc_runner_derive::{aoc, aoc_generator};

use fxhash::{FxHashMap, FxHashSet};


struct LabMap {
    pub tiles: FxHashMap<C2, char>,
    pub guard_start: C2,
    pub guard_direction: C2,
}
#[aoc_generator(day6)]
fn parse(input: &str) -> LabMap {
    let mut tiles = FxHashMap::default();
    let mut guard_start = C2::ZERO;
    let mut guard_direction = C2::ZERO;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = C2::new(x as i32, y as i32);
            match c {
                '.' | '#' => {tiles.insert(coord, c);}
                '^' => {
                    guard_start = coord;
                    guard_direction = C2::new(0, -1);
                    tiles.insert(coord, '.');
                }
                '>' => {
                    guard_start = coord;
                    guard_direction = C2::new(1, 0);
                    tiles.insert(coord, '.');
                }
                'v' => {
                    guard_start = coord;
                    guard_direction = C2::new(0, 1);
                    tiles.insert(coord, '.');
                }
                '<' => {
                    guard_start = coord;
                    guard_direction = C2::new(-1, 0);
                    tiles.insert(coord, '.');
                }
                _ => unimplemented!("Unrecognized character: '{}'", c),
            }
        }
    }

    LabMap {
        tiles,
        guard_start,
        guard_direction,
    }
}

#[aoc(day6, part1)]
fn part1(input: &LabMap) -> i32 {
    let mut visited : FxHashSet<C2>= FxHashSet::default();

    let mut guard_position = input.guard_start;
    let mut guard_direction = input.guard_direction;

    while input.tiles.contains_key(&guard_position) {
        visited.insert(guard_position);
        let next_position = guard_position + guard_direction;
        if input.tiles.get(&next_position) == Some(&'#') {
            guard_direction = guard_direction.rotate_right();
        } else {
            guard_position = next_position;
        }
    }

    visited.len() as i32
}

#[aoc(day6, part2)]
fn part2(input: &LabMap) -> i32 {
    let mut path: Vec<(C2,C2)> = Vec::new();
    let mut guard_position = input.guard_start;
    let mut guard_direction = input.guard_direction;

    while input.tiles.contains_key(&guard_position) {
        path.push((guard_position,guard_direction));
        let next_position = guard_position + guard_direction;
        if input.tiles.get(&next_position) == Some(&'#') {
            guard_direction = guard_direction.rotate_right();
        } else {
            guard_position = next_position;
        }
    }

    let mut successful_obstacles: FxHashSet<C2> = FxHashSet::default();
    let mut failed_obstacles: FxHashSet<C2> = FxHashSet::default();

    'path: for i in 1..path.len() {
        let new_obstacle = path[i].0;
        if successful_obstacles.contains(&new_obstacle) || failed_obstacles.contains(&new_obstacle) {
            continue 'path;
        }
        guard_position = path[i-1].0;
        guard_direction = path[i-1].1;

        let mut visited: FxHashSet<(C2,C2)> = path.iter().take(i-1).cloned().collect();
        
        while input.tiles.contains_key(&guard_position) {
            if !visited.insert((guard_position, guard_direction)) {
                successful_obstacles.insert(new_obstacle);
                continue 'path;
            }
            let next_position = guard_position + guard_direction;
            if input.tiles.get(&next_position) == Some(&'#') || next_position == new_obstacle {
                guard_direction = guard_direction.rotate_right();
            } else {
                guard_position = next_position;
            }
        }

        failed_obstacles.insert(new_obstacle);
    }

    successful_obstacles.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_DATA: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_DATA)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST_DATA)), 6);
    }
}

use crate::utils::c2::{C2Field, C2, C2_DOWN, C2_LEFT, C2_RIGHT, C2_UP};
use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
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

const PRICE_ROTATE: u32 = 1000;
const PRICE_WALK: u32 = 1;
type Map = (C2Field<bool>, C2, C2);
#[aoc_generator(day16)]
fn parse(input: &str) -> Map {
    let mut s = C2::new(-1, -1);
    let mut e = C2::new(-1, -1);

    let map = C2Field::from_string_indexed(input, &mut |c, ch| match ch {
        'S' => {
            s = c;
            true
        }
        'E' => {
            e = c;
            true
        }
        '.' => true,
        _ => false,
    });

    (map, s, e)
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
struct Move {
    position: C2,
    direction: C2,
}

impl Move {
    fn get_possible_moves(&self, map: &C2Field<bool>) -> Vec<(Move, u32)> {
        let mut moves = Vec::with_capacity(3);
        moves.push((
            Move {
                position: self.position,
                direction: self.direction.rotate_right(),
            },
            PRICE_ROTATE,
        ));

        moves.push((
            Move {
                position: self.position,
                direction: self.direction.rotate_left(),
            },
            PRICE_ROTATE,
        ));

        let walk = self.position + self.direction;
        if map.get(&walk) == Some(&true) {
            moves.push((
                Move {
                    position: walk,
                    direction: self.direction,
                },
                PRICE_WALK,
            ))
        }
        moves
    }

    fn get_possible_reverse_moves(&self, map: &C2Field<bool>) -> Vec<(Move, u32)> {
        let mut moves = Vec::with_capacity(3);
        moves.push((
            Move {
                position: self.position,
                direction: self.direction.rotate_right(),
            },
            PRICE_ROTATE,
        ));

        moves.push((
            Move {
                position: self.position,
                direction: self.direction.rotate_left(),
            },
            PRICE_ROTATE,
        ));

        let walk = self.position - self.direction;
        if map.get(&walk) == Some(&true) {
            moves.push((
                Move {
                    position: walk,
                    direction: self.direction,
                },
                PRICE_WALK,
            ))
        }
        moves
    }
}

// 81404 too high
#[aoc(day16, part1)]
fn part1_solution(input: &Map) -> u32 {
    let map = input.0.clone();
    let direction = C2_RIGHT;
    let start = input.1;
    let end = input.2;

    let end_moves = [
        Move {
            position: end,
            direction: C2_UP,
        },
        Move {
            position: end,
            direction: C2_DOWN,
        },
        Move {
            position: end,
            direction: C2_LEFT,
        },
        Move {
            position: end,
            direction: C2_RIGHT,
        },
    ];

    let mut moves_to_check: Vec<Move> = Vec::with_capacity((map.width() / 2) * (map.height() / 2));
    let mut prices: FxHashMap<Move, u32> = FxHashMap::default();

    let start_move = Move {
        position: start,
        direction,
    };
    moves_to_check.push(start_move);
    prices.insert(start_move, 0);

    while let Some(m) = moves_to_check.pop() {
        let p = *prices.get(&m).expect("Should be present");
        for next in m.get_possible_moves(&map) {
            let next_price = p + next.1;
            if let Some(pn) = prices.get(&next.0) {
                if next_price < *pn {
                    prices.insert(next.0, next_price);
                    moves_to_check.push(next.0)
                }
            } else {
                prices.insert(next.0, next_price);
                moves_to_check.push(next.0)
            }
        }
        moves_to_check.sort_by(|a, b| prices[b].partial_cmp(&prices[a]).unwrap_or(Ordering::Equal));
    }
    *end_moves
        .iter()
        .filter_map(|m| prices.get(m))
        .min()
        .expect("Not reached end")
}

fn solve(start: C2, map: &C2Field<bool>) -> FxHashMap<Move, u32> {
    let direction = C2_RIGHT;

    let mut moves_to_check: Vec<Move> = Vec::with_capacity((map.width() / 2) * (map.height() / 2));
    let mut prices: FxHashMap<Move, u32> = FxHashMap::default();

    let start_move = Move {
        position: start,
        direction,
    };
    moves_to_check.push(start_move);
    prices.insert(start_move, 0);

    while let Some(m) = moves_to_check.pop() {
        let p = *prices.get(&m).expect("Should be present");
        for next in m.get_possible_moves(map) {
            let next_price = p + next.1;
            if let Some(pn) = prices.get(&next.0) {
                if next_price < *pn {
                    prices.insert(next.0, next_price);
                    moves_to_check.push(next.0)
                }
            } else {
                prices.insert(next.0, next_price);
                moves_to_check.push(next.0)
            }
        }
        moves_to_check.sort_by(|a, b| prices[b].partial_cmp(&prices[a]).unwrap_or(Ordering::Equal));
    }

    prices
}
#[aoc(day16, part2)]
fn part2_solution(input: &Map) -> u32 {
    let map = input.0.clone();
    let start = input.1;
    let end = input.2;

    let end_moves = [
        Move {
            position: end,
            direction: C2_UP,
        },
        Move {
            position: end,
            direction: C2_DOWN,
        },
        Move {
            position: end,
            direction: C2_LEFT,
        },
        Move {
            position: end,
            direction: C2_RIGHT,
        },
    ];

    let costs = solve(start, &map);
    let target_cost = *end_moves
        .iter()
        .filter_map(|m| costs.get(m))
        .min()
        .expect("Not reached end");

    let mut backtrack: Vec<(Move, u32)> = Vec::with_capacity(costs.len());
    let mut path: FxHashSet<C2> = FxHashSet::default();

    backtrack.extend(
        end_moves
            .iter()
            .filter(|e| costs.get(e) == Some(&target_cost))
            .map(|m| (*m, target_cost)),
    );

    while let Some((m, cost)) = backtrack.pop() {
        path.insert(m.position);
        for (rev_move, move_cost) in m.get_possible_reverse_moves(&map) {
            if cost >= move_cost {
                let previous_cost = cost - move_cost;
                if costs.get(&rev_move) == Some(&previous_cost) {
                    backtrack.push((rev_move, previous_cost));
                }
            }
        }
    }

    let display_map = map.map(|c, b| {
        if path.contains(c) {
            'O'
        } else if *b {
            '.'
        } else {
            '#'
        }
    });

    println!("{}", display_map);

    path.len() as u32
}
// 483
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

    const EXAMPLE_2: &str = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;

    const EXAMPLE_3: &str = r#"################
################
#ES............#
################"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(EXAMPLE)), 7036);
    }
    #[test]
    fn part1_example_2() {
        assert_eq!(part1_solution(&parse(EXAMPLE_2)), 11048);
    }
    #[test]
    fn part1_example_3() {
        assert_eq!(part1_solution(&parse(EXAMPLE_3)), 2001);
    }
    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(EXAMPLE)), 45);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2_solution(&parse(EXAMPLE_2)), 64);
    }
}

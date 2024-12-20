use crate::utils::c2::{C2Field, C2, C2_DOWN, C2_LEFT, C2_RIGHT, C2_UP};
use crate::utils::maze::{parse_maze, Maze};
use aoc_runner_derive::{aoc, aoc_generator};
use binary_heap_plus::BinaryHeap;
use fxhash::{FxHashMap, FxHashSet};
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
#[aoc_generator(day16)]
fn parse(input: &str) -> Maze {
    parse_maze(input)
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

fn solve(start: C2, map: &C2Field<bool>) -> FxHashMap<Move, u32> {
    let direction = C2_RIGHT;

    let mut prices: FxHashMap<Move, u32> = FxHashMap::default();
    let mut moves_to_check = BinaryHeap::new_by_key(|(c, _)| *c);

    let start_move = Move {
        position: start,
        direction,
    };

    moves_to_check.push((0, start_move));
    prices.insert(start_move, 0);

    while let Some((_, m)) = moves_to_check.pop() {
        let p = prices[&m];

        for (next_move, cost) in m.get_possible_moves(map) {
            let next_price = p + cost;

            if let Some(&existing_price) = prices.get(&next_move) {
                if next_price < existing_price {
                    prices.insert(next_move, next_price);
                    moves_to_check.push((!next_price, next_move));
                }
            } else {
                prices.insert(next_move, next_price);
                moves_to_check.push((!next_price, next_move));
            }
        }
    }

    prices
}

fn end_moves(end: C2) -> [Move; 4] {
    [
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
    ]
}

#[aoc(day16, part1)]
fn part1_solution(input: &Maze) -> u32 {
    let map = &input.0;
    let start = input.1;
    let end = input.2;

    let prices = solve(start, map);

    *end_moves(end)
        .iter()
        .filter_map(|m| prices.get(m))
        .min()
        .expect("Not reached end")
}

#[aoc(day16, part2)]
fn part2_solution(input: &Maze) -> u32 {
    let map = &input.0;
    let start = input.1;
    let end = input.2;

    let end_moves = end_moves(end);

    let costs = solve(start, map);
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
        for (rev_move, move_cost) in m.get_possible_reverse_moves(map) {
            if cost >= move_cost {
                let previous_cost = cost - move_cost;
                if costs.get(&rev_move) == Some(&previous_cost) {
                    backtrack.push((rev_move, previous_cost));
                }
            }
        }
    }

    path.len() as u32
}

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

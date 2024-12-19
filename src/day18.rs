use crate::utils::binary_find;
use crate::utils::c2::{C2Field, C2};
use aoc_runner_derive::{aoc, aoc_generator};
use binary_heap_plus::BinaryHeap;
use prse::parse;
use std::cmp::Ordering;

// CodSpeed compatibility
#[allow(dead_code, clippy::useless_format)]
pub fn part1(input: &str) -> String {
    format!("{}",part1_solution(&parse(input)))
}
#[allow(dead_code, clippy::useless_format)]
pub fn part2(input: &str) -> String {
    format!("{}", part2_solution(&parse(input)))
}
// CodSpeed compatibility end

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<C2> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (x, y): (i32, i32) = parse!(l, "{},{}");
            C2::new(x, y)
        })
        .collect()
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct TravelEntry {
    coord: C2,
    moves: u16,
}

impl Ord for TravelEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let moves_cmp = (!self.moves).cmp(&(!other.moves));

        if moves_cmp == Ordering::Equal {
            let a = self.coord.x + self.coord.y;
            let b = other.coord.x + other.coord.y;
            a.cmp(&b)
        } else {
            moves_cmp
        }
    }
}

impl TravelEntry {
    fn new(coord: C2, moves: u16) -> Self {
        Self { coord, moves }
    }
}

impl PartialOrd for TravelEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_min_moves(obstacles: &[C2], obstacle_limit: usize, size: usize) -> u16 {
    let finish = C2::new((size - 1) as i32, (size - 1) as i32);
    let mut field: C2Field<u16> = C2Field::new(size, size);
    for c in obstacles.iter().take(obstacle_limit) {
        field.set(c, u16::MAX);
    }

    let mut to_visit: BinaryHeap<TravelEntry> = BinaryHeap::new();
    to_visit.push(TravelEntry::new(C2::new(0, 0), 0));

    while let Some(t) = to_visit.pop() {
        let c = t.coord;
        if let Some(moves) = field.get(&c) {
            if *moves == 0 || t.moves <= *moves {
                let moves = moves + 1;
                for n in c.neighbors_4() {
                    if n == finish {
                        return moves;
                    }
                    if let Some(n_moves) = field.get(&n) {
                        if *n_moves == 0 || (*n_moves != u16::MAX && *n_moves > moves) {
                            field.set(&n, moves);
                            to_visit.push(TravelEntry::new(n, moves));
                        }
                    }
                }
            }
        }
    }

    u16::MAX
}

#[aoc(day18, part1)]
fn part1_solution(input: &[C2]) -> u16 {
    find_min_moves(input, 1024, 71)
}

fn find_blocking(obstacles: &[C2], limit: usize, size: usize) -> String {
    let low = limit;
    let high = obstacles.len();
    let will_be_stuck = |limit: usize| find_min_moves(obstacles, limit + 1, size) != u16::MAX;
    let block = binary_find(low, high, will_be_stuck);
    let block_coord = obstacles[block];
    format!("{},{}", block_coord.x, block_coord.y)
}

#[aoc(day18, part2)]
fn part2_solution(input: &[C2]) -> String {
    find_blocking(input, 1024, 71)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SMALL_EXAMPLE: &str = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
    #[test]
    fn part1_example() {
        assert_eq!(find_min_moves(&parse(SMALL_EXAMPLE), 12, 7), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(find_blocking(&parse(SMALL_EXAMPLE), 12, 7), "6,1");
    }
}

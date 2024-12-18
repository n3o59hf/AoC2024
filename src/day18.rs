use crate::utils::c2::{C2Field, C2};
use aoc_runner_derive::{aoc, aoc_generator};
use prse::parse;
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

fn solve_1(obstacles: &[C2], limit: usize, size: usize) -> i32 {
    let mut field: C2Field<i32> = C2Field::new(size, size);
    for c in obstacles.iter().take(limit) {
        field.set(c, -1);
    }

    let mut to_visit: Vec<C2> = Vec::new();
    to_visit.push(C2::new(0, 0));

    while let Some(c) = to_visit.pop() {
        if let Some(moves) = field.get(&c) {
            if *moves != -1 {
                let moves = moves + 1;
                for n in c.neighbors_4() {
                    if let Some(n_moves) = field.get(&n) {
                        if *n_moves > moves || *n_moves == 0 {
                            field.set(&n, moves);
                            to_visit.push(n);
                        }
                    }
                }
            }
        }
    }
    *field
        .get(&C2::new((size - 1) as i32, (size - 1) as i32))
        .expect("Should exist")
}

#[aoc(day18, part1)]
fn part1_solution(input: &[C2]) -> i32 {
    solve_1(input, 1024, 71)
}

fn solve_2(obstacles: &[C2], limit: usize, size: usize) -> String {
    for (i, c) in obstacles.iter().enumerate().skip(limit) {
        if solve_1(obstacles, i + 1, size) == 0 {
            return format!("{},{}", c.x, c.y);
        }
    }
    panic!("Should have found the solution")
}
#[aoc(day18, part2)]
fn part2_solution(input: &[C2]) -> String {
    solve_2(input, 1024, 71)
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
        assert_eq!(solve_1(&parse(SMALL_EXAMPLE), 12, 7), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_2(&parse(SMALL_EXAMPLE), 12, 7), "6,1");
    }
}

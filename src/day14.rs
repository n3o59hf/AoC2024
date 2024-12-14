use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
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
type Robot = (i32, i32, i32, i32);
const W: i32 = 101;
const H: i32 = 103;
const TURNS: i32 = 100;

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter_map(|line| {
            if let Some(space) = line.find(' ') {
                let p = &line[2..space];
                let v = &line[(space + 3)..];
                let pc = p.find(',').expect("Missing comma");
                let vc = v.find(',').expect("Missing comma");
                let x: i32 = p[..pc].parse().expect("Data error");
                let y: i32 = p[pc + 1..].parse().expect("Data error");
                let vx: i32 = v[..vc].parse().expect("Data error");
                let vy: i32 = v[vc + 1..].parse().expect("Data error");

                Some((x, vx, y, vy))
            } else {
                None
            }
        })
        .collect()
}

fn part1_solver(input: &[Robot], w: i32, h: i32) -> usize {
    let w_div = w / 2;
    let h_div = h / 2;

    let mut q = [0, 0, 0, 0];
    for robot in input.iter() {
        let x = (robot.0 + (robot.1 + w) * TURNS) % w;
        let y = (robot.2 + (robot.3 + h) * TURNS) % h;

        if x != w_div && y != h_div {
            let q_ind = if x < w_div { 1 } else { 0 } + if y < h_div { 2 } else { 0 };
            q[q_ind] += 1;
        }
    }

    q.into_iter().reduce(|a, b| a * b).expect("Data error")
}

#[aoc(day14, part1)]
fn part1_solution(input: &[Robot]) -> usize {
    part1_solver(input, W, H)
}

#[aoc(day14, part2)]
fn part2_solution(input: &[Robot]) -> i32 {
    let turns = (0..(W * H)).collect::<Vec<i32>>();
    *turns
        .par_iter()
        .map(|turns| {
            let mut seen_map = vec![false; (W * H) as usize];
            let seen: usize = input
                .iter()
                .map(|robot| {
                    let x = (robot.0 + (robot.1 + W) * turns) % W;
                    let y = (robot.2 + (robot.3 + H) * turns) % H;
                    let index = (x + y * W) as usize;
                    if seen_map[index] {
                        0
                    } else {
                        seen_map[index] = true;
                        1
                    }
                })
                .sum();
            (turns, seen)
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .expect("Data error")
        .0
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_DATA: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1_solver(&parse(EXAMPLE_DATA), 11, 7), 12);
    }
}

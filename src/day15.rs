use std::fmt::Display;
use crate::utils::c2::{C2Field, C2, C2_DOWN, C2_LEFT, C2_RIGHT, C2_UP};
use aoc_runner_derive::{aoc, aoc_generator};

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

type Input = (C2Field<char>, Vec<C2>);

const BOX: char = 'O';
const LBOX_L: char = '[';
const LBOX_R: char = ']';
const WALL: char = '#';

const ROBOT: char = '@';

const FLOOR: char = '.';
#[aoc_generator(day15)]
fn parse(input: &str) -> Input {
    let (map, moves) = input.split_once("\n\n").expect("Bad input");

    let map = C2Field::from_string(map, |c| c);
    let moves = moves
        .chars()
        .filter_map(|c| match c {
            '<' => Some(C2_LEFT),
            '^' => Some(C2_UP),
            '>' => Some(C2_RIGHT),
            'v' => Some(C2_DOWN),
            _ => None,
        })
        .collect::<Vec<_>>();

    (map, moves)
}

fn can_move(map: &mut C2Field<char>, position: C2, direction: &C2) -> bool {
    let target = position + *direction;

    let can_move = match *map.get(&target).expect("Should exist") {
        FLOOR => true,
        WALL => false,
        BOX => can_move(map, target, direction),
        LBOX_L => {
            if direction.y == 0 {
                can_move(map, target, direction)
            } else {
                can_move(map, target, direction) && can_move(map, target + C2_RIGHT, direction)
            }
        }
        LBOX_R => {
            if direction.y == 0 {
                can_move(map, target, direction)
            } else {
                can_move(map, target, direction) && can_move(map, target + C2_LEFT, direction)
            }
        }

        _ => panic!("Bad data"),
    };

    can_move
}
fn do_move(map: &mut C2Field<char>, position: C2, direction: &C2, sidestep: bool) {
    let tile = *map.get(&position).expect("Should exist");

    let target = position + *direction;
    match tile {
        FLOOR => return,
        WALL => panic!("Walls are immovable"),
        BOX => do_move(map, target, direction, false),
        LBOX_L | LBOX_R => {
            if direction.y == 0 {
                do_move(map, target, direction, false);
            } else {
                do_move(map, target, direction, false);
                if !sidestep {
                    do_move(
                        map,
                        position + if tile == LBOX_L { C2_RIGHT } else { C2_LEFT },
                        direction,
                        true,
                    );
                }
            }
        }
        _ => panic!("Bad data"),
    }

    let current = *map.get(&position).expect("Should exist");
    let next = *map.get(&target).expect("Should exist");
    map.set(&target, current);
    map.set(&position, next);
}

fn perform_move(map: &mut C2Field<char>, position: C2, direction: &C2) -> bool {
    let can_move = can_move(map, position, direction);
    if can_move {
        do_move(map, position + *direction, direction, false);
    }
    can_move
}

fn solve(input: &Input) -> i32 {
    let mut map = input.0.clone();

    let mut robot = map.find_first(ROBOT).expect("No robot found");
    map.set(&robot, FLOOR);

    for direction in input.1.iter() {
        if perform_move(&mut map, robot, direction) {
            robot = robot + *direction;
        }
    }

    map.iter()
        .filter(|(_, c)| **c == BOX || **c == LBOX_L)
        .map(|(c, _)| c.x + c.y * 100)
        .sum()
}
#[aoc(day15, part1)]
fn part1_solution(input: &Input) -> i32 {
    solve(input)
}

fn enlarge(input: &Input) -> Input {
    let map = input.0.clone();
    let mut new_map = C2Field::new(map.width() * 2, map.height());

    for (c, v) in map.iter() {
        let l = C2::new(c.x * 2, c.y);
        let r = l + C2_RIGHT;
        match *v {
            BOX => {
                new_map.set(&l, LBOX_L);
                new_map.set(&r, LBOX_R);
            }
            WALL => {
                new_map.set(&l, WALL);
                new_map.set(&r, WALL);
            }
            ROBOT => {
                new_map.set(&l, ROBOT);
                new_map.set(&r, FLOOR);
            }
            FLOOR => {
                new_map.set(&l, FLOOR);
                new_map.set(&r, FLOOR);
            }
            _ => panic!("Bad data"),
        }
    }

    (new_map, input.1.clone())
}
#[aoc(day15, part2)]
fn part2_solution(input: &Input) -> i32 {
    let input = enlarge(input);
    solve(&input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(EXAMPLE)), 10092);
    }

    const DOUBLE_STACK: &str = r#"#####
#####
#...#
#.O.#
#.O.#
#.@.#
#####

^"#;
    #[test]
    fn double_stack_move() {
        assert_eq!(part1_solution(&parse(DOUBLE_STACK)), 504);
    }
    const LARGE_BOX: &str = r#"#####
#####
#...#
#.[]#
#.@.#
#####

^"#;
    #[test]
    fn large_box_move() {
        assert_eq!(part1_solution(&parse(LARGE_BOX)), 202);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(EXAMPLE)), 9021);
    }
}

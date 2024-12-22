use crate::utils::c2::{C2, C2_DOWN, C2_LEFT, C2_RIGHT, C2_UP};
use aoc_runner_derive::{aoc, aoc_generator};
use cached::once_cell::sync::Lazy;
use cached::proc_macro::cached;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use std::collections::VecDeque;
use std::string::ToString;

// CodSpeed compatibility
#[allow(dead_code, clippy::useless_format)]
pub fn part1(input: &str) -> String {
    format!("{}", part1_solution(&parse(input)))
}
#[allow(dead_code, clippy::useless_format)]
pub fn part2(input: &str) -> String {
    format!("{}", part2_solution(&parse(input)))
}
// CodSpeed compatibility end

type Input = Vec<String>;

const KEYPAD_BUTTONS: [(C2, char); 11] = [
    (C2::new(0, 0), '7'),
    (C2::new(1, 0), '8'),
    (C2::new(2, 0), '9'),
    (C2::new(0, 1), '4'),
    (C2::new(1, 1), '5'),
    (C2::new(2, 1), '6'),
    (C2::new(0, 2), '1'),
    (C2::new(1, 2), '2'),
    (C2::new(2, 2), '3'),
    (C2::new(1, 3), '0'),
    (C2::new(2, 3), 'A'),
];

static KEYPAD_TRANSITIONS: Lazy<FxHashMap<(char, char), Vec<String>>> =
    Lazy::new(|| calculate_all_transitions(KEYPAD_BUTTONS.as_slice()));
static ROBOT_TRANSITIONS: Lazy<FxHashMap<(char, char), String>> = Lazy::new(|| {
    FxHashMap::from_iter(vec![
        (('^', '^'), "A".to_string()),
        (('^', 'A'), ">A".to_string()),
        (('^', '<'), "v<A".to_string()),
        (('^', 'v'), "vA".to_string()),
        (('^', '>'), "v>A".to_string()),
        (('A', '^'), "<A".to_string()),
        (('A', 'A'), "A".to_string()),
        (('A', '<'), "v<<A".to_string()),
        (('A', 'v'), "<vA".to_string()),
        (('A', '>'), "vA".to_string()),
        (('<', '^'), ">^A".to_string()),
        (('<', 'A'), ">>^A".to_string()),
        (('<', '<'), "A".to_string()),
        (('<', 'v'), ">A".to_string()),
        (('<', '>'), ">>A".to_string()),
        (('v', '^'), "^A".to_string()),
        (('v', 'A'), "^>A".to_string()),
        (('v', '<'), "<A".to_string()),
        (('v', 'v'), "A".to_string()),
        (('v', '>'), ">A".to_string()),
        (('>', '^'), "<^A".to_string()),
        (('>', 'A'), "^A".to_string()),
        (('>', '<'), "<<A".to_string()),
        (('>', 'v'), "<A".to_string()),
        (('>', '>'), "A".to_string()),
    ])
});

#[aoc_generator(day21)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.chars().collect::<String>())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
}

fn find_shortest_paths(a: C2, b: C2, allowed: &FxHashSet<C2>) -> Vec<String> {
    if a == b {
        return vec!["A".to_string()];
    }

    let directions = vec![
        ('^', C2_UP),
        ('>', C2_RIGHT),
        ('v', C2_DOWN),
        ('<', C2_LEFT),
    ];
    let mut results = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back((a, "".to_string(), FxHashSet::default()));

    let mut shortest_distance = None;

    while let Some((current, path, visited)) = queue.pop_front() {
        if let Some(min_dist) = shortest_distance {
            if path.len() > min_dist {
                continue;
            }
        }

        for &(dir, cdir) in &directions {
            let next = current + cdir;
            if allowed.contains(&next) && !visited.contains(&next) {
                let mut new_path = path.clone();
                new_path.push(dir);

                if next == b {
                    new_path.push('A');
                    let path_len = new_path.len();

                    if shortest_distance.is_none() || path_len < shortest_distance.unwrap() {
                        shortest_distance = Some(path_len);
                        results.clear();
                    }

                    if shortest_distance == Some(path_len) {
                        results.push(new_path);
                    }
                } else {
                    let mut new_visited = visited.clone();
                    new_visited.insert(next);
                    queue.push_back((next, new_path, new_visited));
                }
            }
        }
    }

    results
}

fn calculate_all_transitions(data: &[(C2, char)]) -> FxHashMap<(char, char), Vec<String>> {
    let allowed_positions: FxHashSet<_> = data.iter().map(|(c, _)| *c).collect();
    let mut transitions = FxHashMap::default();
    for &(ac, a) in data.iter() {
        for &(bc, b) in data.iter() {
            let key = (a, b);
            let paths = find_shortest_paths(ac, bc, &allowed_positions);
            transitions.insert(key, paths);
        }
    }

    transitions
}

fn process_keypad_transitions(input: String) -> FxHashSet<String> {
    let mut combinations: Vec<String> = vec!["".to_string()];

    for transition in format!("A{input}").chars().collect_vec().windows(2) {
        if let &[from, to] = transition {
            let transitions = KEYPAD_TRANSITIONS.get(&(from, to)).expect("Transition");

            combinations = combinations
                .iter()
                .flat_map(|a| transitions.iter().map(|b| a.clone() + b))
                .collect_vec();
        }
    }

    FxHashSet::from_iter(combinations)
}

fn get_numeric(vec: &str) -> u64 {
    let mut number = 0;
    for v in vec.chars() {
        if let Some(digit) = v.to_digit(10) {
            number = number * 10 + digit;
        }
    }
    number as u64
}

fn calculate_cost(data: &String, depth: u8) -> u64 {
    let full_move = format!("A{data}");

    let result = full_move
        .chars()
        .collect_vec()
        .windows(2)
        .map(|w| {
            let b = w[0];
            let c = w[1];
            calculate_robot_cost(b, c, depth - 1)
        })
        .sum::<u64>();
    result
}

#[cached]
fn calculate_robot_cost(b: char, c: char, depth: u8) -> u64 {
    let forward = ROBOT_TRANSITIONS.get(&(b, c)).expect("forward");
    let result = if depth == 0 {
        forward.len() as u64
    } else {
        let full_move = format!("A{forward}").chars().collect_vec();

        let mut sum = 0;

        for i in 0..full_move.len() - 1 {
            let b = full_move[i];
            let c = full_move[i + 1];
            sum += calculate_robot_cost(b, c, depth - 1);
        }
        sum
    };

    result
}

fn solve(input: &Input, depth: u8) -> u64 {
    let mut result = 0;
    for s in input.iter() {
        let min_path = process_keypad_transitions(s.clone())
            .iter()
            .map(|comb| calculate_cost(comb, depth))
            .min()
            .expect("combination");
        result += min_path * get_numeric(s);
    }
    result
}

#[aoc(day21, part1)]
fn part1_solution(input: &Input) -> u64 {
    solve(input, 2)
}

#[aoc(day21, part2)]
fn part2_solution(input: &Input) -> u64 {
    solve(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "029A";
    const EXAMPLE: &str = r#"029A
980A
179A
456A
379A
"#;
    #[test]
    fn part1_example_1() {
        assert_eq!(part1_solution(&parse(EXAMPLE_1)), 68 * 29);
    }
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(EXAMPLE)), 126384);
    }
}

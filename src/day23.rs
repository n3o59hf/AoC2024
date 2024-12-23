use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashSet;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hasher};

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

struct TwoCharHasher {
    hash: u64,
}

impl Hasher for TwoCharHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        if bytes.len() == 2 {
            let c1 = bytes[0];
            let c2 = bytes[1];
            if c1.is_ascii_lowercase() && c2.is_ascii_lowercase() {
                self.hash = (c1 - b'a') as u64 * 26 + (c2 - b'a') as u64;
            } else {
                panic!("Wrong input");
            }
        } else {
            panic!("Wrong input");
        }
    }

    fn write_u8(&mut self, byte: u8) {
        self.hash ^= (byte as u64).swap_bytes();
    }

    fn write_usize(&mut self, u: usize) {
        self.hash ^= u as u64;
    }
}

#[derive(Default, Copy, Clone)]
struct TwoCharHasherBuilder;

impl BuildHasher for TwoCharHasherBuilder {
    type Hasher = TwoCharHasher;

    fn build_hasher(&self) -> Self::Hasher {
        TwoCharHasher { hash: 0 }
    }
}

type S2HashSet = HashSet<String, TwoCharHasherBuilder>;
type Input = HashMap<String, S2HashSet, TwoCharHasherBuilder>;

#[aoc_generator(day23)]
fn parse(input: &str) -> Input {
    let links = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| (l[0..2].to_string(), l[3..5].to_string()));

    let mut connections: Input = Default::default();

    for (a, b) in links {
        connections
            .entry(a.to_string())
            .or_default()
            .insert(b.to_string());
        connections.entry(b).or_default().insert(a);
    }

    connections
}

#[aoc(day23, part1)]
fn part1_solution(input: &Input) -> usize {
    let mut chains: FxHashSet<String> = FxHashSet::default();

    for (first, neighbors) in input {
        if first.starts_with("t") {
            for (second, last) in neighbors.iter().tuple_combinations() {
                if input[second].contains(last) {
                    chains.insert([first, second, last].iter().sorted().join(""));
                }
            }
        }
    }
    chains.len()
}

fn find_largest(
    r: &S2HashSet,
    mut p: S2HashSet,
    mut x: S2HashSet,
    connections: &Input,
    largest_network: &mut Vec<String>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > largest_network.len() {
            largest_network.clear();
            largest_network.extend(r.iter().cloned());
        }
        return;
    }

    let pivot = p
        .union(&x)
        .max_by_key(|v| {
            connections
                .get(*v)
                .map_or(0, |neighbors| neighbors.intersection(&p).count())
        })
        .expect("Exists");

    for v in p.difference(&connections[pivot]).cloned().collect_vec() {
        let mut new_network = r.clone();
        new_network.insert(v.clone());

        if let Some(neighbors) = connections.get(&v) {
            let new_p: S2HashSet = p.intersection(neighbors).cloned().collect();
            if new_network.len() + new_p.len() > largest_network.len() {
                find_largest(
                    &new_network,
                    new_p,
                    x.intersection(neighbors).cloned().collect(),
                    connections,
                    largest_network,
                );
            }
        }

        p.remove(&v);
        x.insert(v.clone());
    }
}

#[aoc(day23, part2)]
fn part2_solution(input: &Input) -> String {
    let mut largest_network: Vec<String> = Vec::new();

    find_largest(
        &Default::default(),
        input.keys().cloned().sorted().collect(),
        Default::default(),
        input,
        &mut largest_network,
    );

    largest_network.sort();
    largest_network.join(",").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(EXAMPLE)), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_solution(&parse(EXAMPLE)), "co,de,ka,ta");
    }
}

use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

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
type Input = FxHashMap<String, FxHashSet<String>>;

#[aoc_generator(day23)]
fn parse(input: &str) -> Input {
    let links = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|l| (l[0..2].to_string(), l[3..5].to_string()));

    let mut connections: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();

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
    let mut chains: FxHashSet<Vec<String>> = FxHashSet::default();

    for (first, neighbors) in input {
        if first.starts_with("t") {
            for (second, last) in neighbors.iter().tuple_combinations() {
                if input[second].contains(last) {
                    let mut chain = vec![first.clone(), second.clone(), last.clone()];
                    chain.sort();
                    chains.insert(chain);
                }
            }
        }
    }
    chains.len()
}

fn find_largest(
    r: &FxHashSet<String>,
    mut p: FxHashSet<String>,
    mut x: FxHashSet<String>,
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
        .unwrap();
    let mut candidates = p.difference(&connections[pivot]).cloned().collect_vec();
    candidates.sort_unstable_by_key(|v| {
        connections
            .get(v)
            .map_or(0, |neighbors| neighbors.intersection(&p).count())
    });
    for v in candidates {
        let mut new_network = r.clone();
        new_network.insert(v.clone());

        if let Some(neighbors) = connections.get(&v) {
            let new_p: FxHashSet<String> = p.intersection(neighbors).cloned().collect();
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

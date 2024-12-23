use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};

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
        .filter_map(|l| l.trim().split_once("-"))
        .map(|(a, b)| (a.to_string(), b.to_string()));

    let mut connections: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();

    for (a, b) in links {
        connections
            .entry(a.to_string())
            .or_default()
            .insert(b.to_string());
        connections
            .entry(b.to_string())
            .or_default()
            .insert(a.to_string());
    }

    connections
}

#[aoc(day23, part1)]
fn part1_solution(input: &Input) -> usize {
    let mut chains: FxHashSet<Vec<String>> = FxHashSet::default();

    for first in input.keys() {
        if let Some(neighbors) = input.get(first) {
            for second in neighbors {
                if let Some(second_neighbors) = input.get(second) {
                    for last in second_neighbors {
                        if last != first && input.get(last).map_or(false, |x| x.contains(first)) {
                            let mut chain = vec![first.clone(), second.clone(), last.clone()];
                            chain.sort();
                            chains.insert(chain);
                        }
                    }
                }
            }
        }
    }
    chains
        .iter()
        .filter(|connection| connection.iter().any(|c| c.starts_with("t")))
        .count()
}

fn find_largest(
    r: &FxHashSet<String>,
    p: &mut FxHashSet<String>,
    x: &mut FxHashSet<String>,
    connections: &Input,
    largest_network: &mut Vec<String>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > largest_network.len() {
            *largest_network = r.iter().cloned().collect();
        }
        return;
    }

    let pivot = p.union(x).next().unwrap();
    let mut candidates: FxHashSet<_> = p.difference(&connections[pivot]).cloned().collect();

    while let Some(v) = candidates.iter().next().cloned() {
        candidates.remove(&v);
        let mut new_network = r.clone();
        new_network.insert(v.clone());

        let neighbors = connections.get(&v).unwrap();
        find_largest(
            &new_network,
            &mut p.intersection(neighbors).cloned().collect(),
            &mut x.intersection(neighbors).cloned().collect(),
            connections,
            largest_network,
        );

        p.remove(&v);
        x.insert(v.clone());
    }
}

#[aoc(day23, part2)]
fn part2_solution(input: &Input) -> String {
    let mut largest_network: Vec<String> = Vec::new();

    find_largest(
        &FxHashSet::default(),
        &mut input.keys().cloned().collect(),
        &mut FxHashSet::default(),
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

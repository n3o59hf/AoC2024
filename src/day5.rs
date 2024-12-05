use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use prse::parse;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
struct Rules {
    order: HashSet<(i32, i32)>,
}

impl Rules {
    fn new(order: Vec<(i32, i32)>) -> Self {
        Self { order: order.into_iter().collect() }
    }
}

impl Rules {
    fn compare(&self, a: &i32, b: &i32) -> Ordering {
        if self.order.contains(&(*a, *b)) {
            Ordering::Less
        } else if self.order.contains(&(*b, *a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Debug)]
struct Page {
    rules: Arc<Rules>,
    number: i32,
}

impl Page {
    fn new(rules: Arc<Rules>, number: i32) -> Self {
        Self { rules, number }
    }
}

impl PartialEq<Self> for Page {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Eq for Page {}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rules.compare(&self.number, &other.number)
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Vec<Vec<Page>> {
    let lines = input.lines().clone().collect::<Vec<&str>>();
    let mut split_lines = lines.split(|l| l.trim().is_empty());

    let mut order: Vec<(i32, i32)> = Vec::new();
    for rule in split_lines.next().unwrap_or(&[]) {
        let (left, right): (i32, i32) = parse!(rule, "{}|{}");
        order.push((left, right))
    }

    let rules = Arc::new(Rules::new(order));

    let mut pages: Vec<Vec<Page>> = Vec::new();

    for update in split_lines.next().unwrap_or(&[]) {
        pages.push(
            update
                .split(",")
                .filter_map(|n| n.parse::<i32>().ok())
                .map(|n| Page::new(rules.clone(), n))
                .collect::<Vec<Page>>(),
        )
    }

    pages
}

#[aoc(day5, part1)]
fn part1(input: &[Vec<Page>]) -> i32 {
    input
        .iter()
        .filter(|pages| pages.is_sorted())
        .map(|pages| pages[pages.len() / 2].number)
        .sum::<i32>()
}

#[aoc(day5, part2)]
fn part2(input: &[Vec<Page>]) -> i32 {
    input
        .iter()
        .filter(|pages| !pages.is_sorted())
        .map(|pages| pages.iter().sorted().collect::<Vec<&Page>>()[pages.len() / 2].number)
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(TEST_DATA)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(TEST_DATA)), 123);
    }
}

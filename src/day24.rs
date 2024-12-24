use crate::day24::Key::{O, X, Y, Z};
use crate::day24::Op::{And, Or, Xor};
use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use prse::try_parse;
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::cmp::{max, Ordering};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Key {
    X(usize),
    Y(usize),
    Z(usize),
    O(String),
}

impl Key {
    fn is_input(&self) -> bool {
        match self {
            X(_) => true,
            Y(_) => true,
            Z(_) => false,
            O(_) => false,
        }
    }

    fn is_output(&self) -> bool {
        match self {
            X(_) => false,
            Y(_) => false,
            Z(_) => true,
            O(_) => false,
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            X(_) => write!(f, "x")?,
            Y(_) => write!(f, "y")?,
            Z(_) => write!(f, "z")?,
            O(_) => {}
        }
        match self {
            X(i) | Y(i) | Z(i) => {
                write!(f, "{:02}", i)
            }
            O(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Operation {
    k1: Key,
    k2: Key,
    op: Op,
    target: Key,
    classification: Option<OperationClassification>,
}

impl Ord for Operation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.target.cmp(&other.target)
    }
}

impl PartialOrd for Operation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Op {
    Or,
    And,
    Xor,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum OperationClassification {
    Suspicious,
    Output,
    InputXor,
    InputAnd,
    Carry,
    Remainder,
}

impl Operation {
    pub fn new(k1: Key, k2: Key, op: Op, target: Key) -> Self {
        Self {
            k1,
            k2,
            op,
            target,
            classification: None,
        }
    }
    pub fn classify(
        &self,
        data: &Input,
        cache: &mut FxHashMap<Operation, OperationClassification>,
    ) -> OperationClassification {
        if let Some(classification) = cache.get(self) {
            *classification
        } else {
            let classification = self.classify_inner(data, cache);
            cache.insert(self.clone(), classification);
            classification
        }
    }

    fn classify_inner(
        &self,
        data: &Input,
        cache: &mut FxHashMap<Operation, OperationClassification>,
    ) -> OperationClassification {
        if self.target.is_output() {
            return if self.op == Xor {
                OperationClassification::Output
            } else if self.op == Or && self.target == Z(data.z_last_index) {
                // Last output is carry
                OperationClassification::Carry
            } else {
                OperationClassification::Suspicious
            };
        } else if self.k1.is_input() && self.k2.is_input() {
            return match self.op {
                Or => OperationClassification::Suspicious,
                And => OperationClassification::InputAnd,
                Xor => OperationClassification::InputXor,
            };
        } else if !self.k1.is_input() && !self.k2.is_input() {
            match self.op {
                Or => {
                    if self.check_parent(
                        OperationClassification::Remainder,
                        OperationClassification::InputAnd,
                        data,
                        cache,
                    ) {
                        return OperationClassification::Carry;
                    }
                }
                And => {
                    if self.check_parent(
                        OperationClassification::Carry,
                        OperationClassification::InputXor,
                        data,
                        cache,
                    ) {
                        return OperationClassification::Remainder;
                    } else if self.check_parent(
                        OperationClassification::InputAnd,
                        OperationClassification::InputXor,
                        data,
                        cache,
                    ) {
                        // First remainder
                        if [&self.k1, &self.k2]
                            .iter()
                            .filter_map(|k| data.operations.get(k))
                            .flat_map(|o| [&o.k1, &o.k2])
                            .filter(|k| k.is_input())
                            .count()
                            == 4
                        {
                            return OperationClassification::Remainder;
                        }
                    }
                }
                Xor => return OperationClassification::Suspicious,
            };
        }

        OperationClassification::Suspicious
    }

    fn check_parent(
        &self,
        op1: OperationClassification,
        op2: OperationClassification,
        data: &Input,
        cache: &mut FxHashMap<Operation, OperationClassification>,
    ) -> bool {
        if let Some(p1) = data.operations.get(&self.k1) {
            if let Some(p2) = data.operations.get(&self.k2) {
                let p1c = p1.classify(data, cache);
                let p2c = p2.classify(data, cache);
                return (p1c == OperationClassification::Suspicious && (p2c == op1 || p2c == op2))
                    || (p2c == OperationClassification::Suspicious && (p1c == op1 || p1c == op2))
                    || (p1c == op1 && p2c == op2)
                    || (p1c == op2 && p2c == op1);
            }
        }

        false
    }
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        let start = &value[0..1];
        let rest = &value[1..];
        match start {
            "x" => X(rest.parse::<usize>().expect("x")),
            "y" => Y(rest.parse::<usize>().expect("y")),
            "z" => Z(rest.parse::<usize>().expect("z")),
            _ => O(value.to_string()),
        }
    }
}

#[derive(Clone, Debug)]
struct Input {
    x: u64,
    y: u64,
    z_last_index: usize,
    operations: FxHashMap<Key, Operation>,
}

impl Input {
    pub fn set_input(&mut self, x: u64, y: u64) {
        self.x = x;
        self.y = y;
    }

    pub fn with_swaps(self, swaps: &[(Key, Key)]) -> Self {
        let mut operations = self.operations.clone();

        for (k1, k2) in swaps.iter().cloned() {
            if let (Some(val1), Some(val2)) = (operations.remove(&k1), operations.remove(&k2)) {
                operations.insert(k1, val2);
                operations.insert(k2, val1);
            }
        }
        Self { operations, ..self }
    }

    pub fn expected_z(&self) -> u64 {
        self.x + self.y
    }
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Input {
    let mut operations: FxHashMap<Key, Operation> = FxHashMap::default();
    let mut x = 0u64;
    let mut y = 0u64;
    let mut z_size = 0usize;
    for line in input.lines() {
        if line.len() == 6 {
            let letter = &line[0..1];
            let index = &line[1..3].parse::<usize>().expect("Couldn't parse index");
            let value = line[5..].parse::<u64>().expect("Couldn't parse value");

            match letter {
                "x" => x += value << index,
                "y" => y += value << index,
                _ => panic!("Unknown letter {letter}"),
            }
        } else {
            let expression_result: Result<(&str, &str, &str, &str), _> =
                try_parse!(line, "{} {} {} -> {}");
            if let Ok((op1, operation, op2, key)) = expression_result {
                if let Some(number) = key.strip_prefix("z") {
                    z_size = max(
                        z_size,
                        number.parse::<usize>().expect("Couldn't parse z_size"),
                    );
                }
                let k1 = op1.into();
                let k2 = op2.into();
                let op = match operation {
                    "OR" => Or,
                    "AND" => And,
                    "XOR" => Xor,
                    _ => panic!("Unknow operation {operation}"),
                };
                operations.insert(key.into(), Operation::new(k1, k2, op, key.into()));
            }
        }
    }
    Input {
        x,
        y,
        z_last_index: z_size,
        operations,
    }
}

fn find_value(key: &Key, data: &Input) -> bool {
    match key {
        X(i) => (data.x & (1u64 << i)) > 0,
        Y(i) => (data.y & (1u64 << i)) > 0,
        _ => {
            let operation = &data.operations[key];
            let k1 = &operation.k1;
            let k2 = &operation.k2;
            match operation.op {
                Or => find_value(k1, data) || find_value(k2, data),
                And => find_value(k1, data) && find_value(k2, data),
                Xor => find_value(k1, data) ^ find_value(k2, data),
            }
        }
    }
}

fn solve(data: &Input) -> u64 {
    let mut z = 0;
    for i in 0..=data.z_last_index {
        if find_value(&Z(i), data) {
            z += 1 << i;
        }
    }
    z
}

#[aoc(day24, part1)]
fn part1_solution(input: &Input) -> u64 {
    solve(input)
}

fn is_ok(input: &Input) -> bool {
    input.expected_z() == solve(input)
}

fn check_bit(bit: usize, input: &Input) -> bool {
    let value = 1u64 << bit;
    let mut input = input.clone();
    input.set_input(value, value);
    if !is_ok(&input) {
        return false;
    }
    input.set_input(value, 0);
    if !is_ok(&input) {
        return false;
    }
    input.set_input(0, value);
    if !is_ok(&input) {
        return false;
    }

    true
}

fn has_cycle(key: &Key, input: &Input, seen: &mut FxHashSet<Key>) -> bool {
    if seen.insert(key.clone()) {
        if let Some(op) = input.operations.get(key) {
            if has_cycle(&op.k1, input, seen) || has_cycle(&op.k2, input, seen) {
                return true;
            }
        }
        seen.remove(key);
        false
    } else {
        true
    }
}
fn verify(swaps: &[(Key, Key)], input: &Input) -> bool {
    let mut parity_check: FxHashSet<Key> = Default::default();
    for (a, b) in swaps.iter().cloned() {
        if !parity_check.insert(a) {
            return false;
        }
        if !parity_check.insert(b) {
            return false;
        }
    }

    let input = input.clone().with_swaps(swaps);

    if parity_check
        .iter()
        .any(|k| has_cycle(k, &input, &mut FxHashSet::default()))
    {
        return false;
    }

    for i in 0..input.z_last_index {
        if !check_bit(i, &input) {
            return false;
        }
    }

    true
}

fn backtrace_suspicious_node(op: &Operation, input: &Input, errors: &mut FxHashSet<Key>) {
    let parents: Vec<&Operation> = [input.operations.get(&op.k1), input.operations.get(&op.k2)]
        .iter()
        .filter_map(|x| *x)
        .collect();

    for parent in parents {
        match op.op {
            Or => {
                if parent.op == Xor || parent.op == Or {
                    errors.insert(parent.target.clone());
                }
            }
            And => {
                if parent.op == And {
                    errors.insert(parent.target.clone());
                }
            }
            Xor => {}
        }
    }
}
fn find_suspicious_nodes(input: &Input) -> FxHashSet<Key> {
    let mut errors = FxHashSet::default();
    let mut cache: FxHashMap<Operation, OperationClassification> = FxHashMap::default();
    for (target, operation) in input.operations.iter().sorted() {
        let classification = operation.classify(input, &mut cache);
        if classification == OperationClassification::Suspicious {
            errors.insert(target.clone());
            backtrace_suspicious_node(operation, input, &mut errors);
        }
    }

    errors
}
#[aoc(day24, part2)]
fn part2_solution(input: &Input) -> String {
    let mut potential_swaps = find_suspicious_nodes(input);

    potential_swaps.retain(|k| !k.is_input());

    let swap = potential_swaps
        .into_iter()
        .sorted()
        .combinations(2)
        .map(|comb| (comb[0].clone(), comb[1].clone()))
        .combinations(4)
        .filter(|combo| {
            let mut used = FxHashSet::default();
            for (a, b) in combo.iter() {
                if !used.insert(a) {
                    return false;
                }
                if !used.insert(b) {
                    return false;
                }
            }
            true
        })
        .par_bridge()
        .find_any(|c| verify(c, input));

    if let Some(swap) = swap {
        return swap
            .into_iter()
            .flat_map(|(a, b)| vec![a, b])
            .sorted()
            .join(",");
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;
    #[test]
    fn part1_example() {
        assert_eq!(part1_solution(&parse(EXAMPLE)), 2024);
    }
}

#![feature(hash_drain_filter)]

use std::collections::HashMap;
use std::fs;

type Pair = (char, char);
type Operations = HashMap<Pair, char>;
type Polymer = HashMap<Pair, usize>;

fn step(polymer: &mut Polymer, operations: &Operations) {
    let new_pairs: Vec<(Pair, usize)> = polymer
        .drain_filter(|pair, _| operations.contains_key(pair))
        .flat_map(|(pair, count)| {
            let insertion = operations.get(&pair).unwrap();
            [((pair.0, *insertion), count), ((*insertion, pair.1), count)]
        })
        .collect();

    for (pair, count) in new_pairs {
        *polymer.entry(pair).or_default() += count;
    }
}

fn run(polymer: &Polymer, operations: &Operations, num: usize) -> usize {
    let mut polymer = polymer.clone();
    for _ in 0..num {
        step(&mut polymer, operations);
    }
    let mut double_counts: HashMap<char, usize> = HashMap::new();
    for ((l, r), count) in polymer {
        *double_counts.entry(l).or_default() += count;
        *double_counts.entry(r).or_default() += count;
    }
    double_counts.remove(&'?');

    (double_counts.values().max().unwrap() - double_counts.values().min().unwrap()) / 2
}

fn part1(polymer: &Polymer, operations: &Operations) -> usize {
    run(polymer, operations, 10)
}

fn part2(polymer: &Polymer, operations: &Operations) -> usize {
    run(polymer, operations, 40)
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let (polymer, operations) = content.split_once("\n\n").unwrap();

    let first = polymer.chars().nth(0).unwrap();
    let last = polymer.chars().nth_back(0).unwrap();

    let polymer = polymer
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|chars| ((chars[0], chars[1]), 1))
        .chain([(('?', first), 1), ((last, '?'), 1)])
        .collect();

    let operations = operations
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" -> ").unwrap();
            (
                (input.chars().nth(0).unwrap(), input.chars().nth(1).unwrap()),
                output.chars().nth(0).unwrap(),
            )
        })
        .collect();

    println!("Part 1: {}", part1(&polymer, &operations));
    println!("Part 2: {}", part2(&polymer, &operations));
}

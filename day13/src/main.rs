#![feature(hash_drain_filter)]

use std::collections::HashSet;

enum Fold {
    X(usize),
    Y(usize),
}

fn fold(points: &mut HashSet<(usize, usize)>, fold: &Fold) -> usize {
    let new_points: Vec<_> = match fold {
        Fold::X(val) => points
            .drain_filter(|(x, _)| x > &val)
            .map(|(x, y)| (2 * val - x, y))
            .collect(),
        Fold::Y(val) => points
            .drain_filter(|(_, y)| y > &val)
            .map(|(x, y)| (x, 2 * val - y))
            .collect(),
    };
    points.extend(new_points);
    points.len()
}

fn part1(points: &HashSet<(usize, usize)>, folds: &Vec<Fold>) -> usize {
    let mut points = points.clone();
    fold(&mut points, &folds[0])
}

fn part2(points: &HashSet<(usize, usize)>, folds: &Vec<Fold>) -> String {
    let mut points = points.clone();
    for instruction in folds {
        fold(&mut points, instruction);
    }
    let x_max = folds
        .iter()
        .filter_map(|fold| match fold {
            Fold::X(val) => Some(val),
            _ => None,
        })
        .last()
        .unwrap();
    let y_max = folds
        .iter()
        .filter_map(|fold| match fold {
            Fold::Y(val) => Some(val),
            _ => None,
        })
        .last()
        .unwrap();
    let mut builder = vec!['\n'];
    for j in 0..*y_max {
        for i in 0..*x_max {
            builder.push(if points.contains(&(i, j)) { '#' } else { '.' });
        }
        builder.push('\n');
    }
    String::from_iter(builder)
}

fn main() {
    let (points, folds) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let points = points
        .lines()
        .map(|line| {
            line.split_once(',').unwrap()
        })
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let folds = folds
        .lines()
        .map(|line| {
            let (axis, value) = line.strip_prefix("fold along ").unwrap().split_once('=').unwrap();
            let value = value.parse().unwrap();
            match axis {
                "x" => Fold::X(value),
                "y" => Fold::Y(value),
                _ => unreachable!(),
            }
        })
        .collect();

    println!("Part 1: {}", part1(&points, &folds));
    println!("Part 2: {}", part2(&points, &folds));
}

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

fn cell_risk(i: usize, j: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let tile_i = (i / grid.len()) as u32;
    let tile_j = (j / grid[0].len()) as u32;

    let base = (grid[i % grid.len()][j % grid[0].len()] + tile_i + tile_j) % 9;
    if base == 0 {
        9
    } else {
        base
    }
}

fn solve(grid: &Vec<Vec<u32>>, factor: usize) -> u32 {
    let height = factor * grid.len();
    let width = factor * grid[0].len();

    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse((0, (0, 0))));
    let mut visited = HashSet::new();

    while !frontier.is_empty() {
        let Reverse((risk, (i, j))) = frontier.pop().unwrap();
        if visited.contains(&(i, j)) {
            continue;
        }
        visited.insert((i, j));

        if (i, j) == (height - 1, width - 1) {
            return risk;
        }
        let mut neighbors = vec![];
        if i > 0 {
            neighbors.push((i - 1, j));
        }
        if j > 0 {
            neighbors.push((i, j - 1));
        }
        if i < height - 1 {
            neighbors.push((i + 1, j));
        }
        if j < width - 1 {
            neighbors.push((i, j + 1));
        }
        frontier.extend(
            neighbors
                .iter()
                .map(|&(i, j)| Reverse((risk + cell_risk(i, j, grid), (i, j)))),
        )
    }

    unreachable!();
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let tile = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    println!("Part 1: {}", solve(&tile, 1));
    println!("Part 2: {}", solve(&tile, 5));
}

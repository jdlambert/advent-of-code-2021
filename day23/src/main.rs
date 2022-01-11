use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

type Move = (usize, usize, usize, usize);

static ROOMS: &[usize] = &[2, 4, 6, 8];
static STOPS: &[usize] = &[0, 1, 3, 5, 7, 9, 10];

fn weight(pod: usize) -> usize {
    match pod {
        2 => 1,
        4 => 10,
        6 => 100,
        8 => 1000,
        _ => unreachable!(),
    }
}

fn solved(grid: &[Vec<usize>]) -> bool {
    grid.iter()
        .enumerate()
        .all(|(i, col)| col.iter().all(|&cell| cell == 0 || cell == i))
}

fn move_cost((i0, j0, i1, j1): Move, grid: &[Vec<usize>]) -> usize {
    let pod = grid[i0][j0].max(grid[i1][j1]);
    (i0.max(i1) - i0.min(i1) + j0.max(j1) - j0.min(j1)) * weight(pod)
}

fn unobstructed(grid: &[Vec<usize>], hall_i: usize, room_i: usize, skip_hall: bool) -> bool {
    (hall_i.min(room_i)..=hall_i.max(room_i)).all(|i| (skip_hall && i == hall_i) || grid[i][0] == 0)
}

fn in_j(grid: &[Vec<usize>], pod: usize) -> Option<usize> {
    let col = &grid[pod as usize];
    if col.iter().any(|&cell| cell != 0 && cell != pod) {
        return None;
    }
    col.iter()
        .enumerate()
        .take_while(|(_, &c)| c == 0)
        .map(|(i, _)| i)
        .last()
}

fn out_j(grid: &[Vec<usize>], pod: usize) -> Option<usize> {
    let col = &grid[pod as usize];
    if col.iter().all(|&cell| cell == 0 || cell == pod) {
        return None;
    }
    for (i, &cell) in col.iter().enumerate() {
        if cell != 0 {
            return Some(i);
        }
    }
    None
}

fn moves(grid: &[Vec<usize>]) -> Vec<Move> {
    for &stop in STOPS {
        let pod = grid[stop][0];
        if pod != 0 && unobstructed(grid, stop, pod, true) {
            if let Some(j) = in_j(grid, pod) {
                return vec![(stop, 0, pod, j)]; // If an "in move" is available, just do it
            }
        }
    }
    let mut moves = vec![];
    for &room in ROOMS { // determine all "out moves"
        if let Some(j) = out_j(grid, room) {
            for &stop in STOPS {
                if unobstructed(grid, stop, room, false) {
                    moves.push((room, j, stop, 0));
                }
            }
        }
    }
    moves
}

fn apply_move(mut grid: Vec<Vec<usize>>, (i0, j0, i1, j1): Move) -> Vec<Vec<usize>> {
    grid[i1][j1] = grid[i0][j0];
    grid[i0][j0] = 0;
    grid
}

fn solve(grid: Vec<Vec<usize>>) -> usize {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    heap.push(Reverse((0, grid)));

    while !heap.is_empty() {
        let Reverse((cost, grid)) = heap.pop().unwrap();
        if !seen.insert(grid.clone()) {
            continue;
        }
        if solved(&grid) {
            return cost;
        }
        for m in moves(&grid) {
            let new_grid = apply_move(grid.clone(), m);
            if seen.contains(&new_grid) {
                continue;
            }
            let new_cost = cost + move_cost(m, &grid);
            heap.push(Reverse((new_cost, new_grid)));
        }
    }
    unreachable!();
}

fn part1(grid: &[Vec<usize>]) -> usize {
    solve(grid.to_owned())
}

fn part2(grid: &[Vec<usize>]) -> usize {
    let mut new_grid = grid.to_owned();
    new_grid[2].insert(2, 8);
    new_grid[2].insert(3, 8);
    new_grid[4].insert(2, 6);
    new_grid[4].insert(3, 4);
    new_grid[6].insert(2, 4);
    new_grid[6].insert(3, 2);
    new_grid[8].insert(2, 2);
    new_grid[8].insert(3, 6);
    solve(new_grid)
}

fn main() {
    let mut grid = vec![vec![0]; 11];
    for line in include_str!("../input.txt").lines().skip(2) {
        for (i, ch) in line.chars().filter(char::is_ascii_uppercase).enumerate() {
            grid[(i + 1) * 2].push((ch as usize - 'A' as usize + 1) * 2);
        }
    }
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

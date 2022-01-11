use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

type Move = (usize, usize, usize, usize);

static ROOMS: &[usize] = &[2, 4, 6, 8];
static STOPS: &[usize] = &[0, 1, 3, 5, 7, 9, 10];

#[allow(dead_code)]
fn rep(pod: u32) -> char {
    match pod {
        0 => '.',
        2 => 'A',
        4 => 'B',
        6 => 'C',
        8 => 'D',
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<u32>]) {
    println!("#############");
    print!("#");
    for col in grid {
        print!("{}", rep(col[0]));
    }
    println!("#");
    println!(
        "###{}#{}#{}#{}###",
        rep(grid[2][1]),
        rep(grid[4][1]),
        rep(grid[6][1]),
        rep(grid[8][1])
    );
    for j in 2..grid[2].len() {
        println!(
            "  #{}#{}#{}#{}#",
            rep(grid[2][j]),
            rep(grid[4][j]),
            rep(grid[6][j]),
            rep(grid[8][j])
        );
    }
    println!("  #########");
}

fn weight(pod: u32) -> u32 {
    match pod {
        2 => 1,
        4 => 10,
        6 => 100,
        8 => 1000,
        _ => unreachable!(),
    }
}

fn solved(grid: &[Vec<u32>]) -> bool {
    grid.iter()
        .enumerate()
        .all(|(i, col)| col.iter().all(|&cell| cell == 0 || cell == i as u32))
}

fn estimate_pod(pod: u32, i: u32, j: u32) -> u32 {
    if pod == 0 || pod == i {
        return 0;
    }
    (pod.max(i) - pod.min(i) + j) * weight(pod)
}

fn move_cost((i0, j0, i1, j1): Move, pod: u32) -> u32 {
    (i0.max(i1) - i0.min(i1) + j0.max(j1) - j0.min(j1)) as u32 * weight(pod)
}

fn estimate(grid: &[Vec<u32>]) -> u32 {
    grid.iter()
        .enumerate()
        .flat_map(|(i, col)| {
            col.iter()
                .enumerate()
                .map(move |(j, &pod)| estimate_pod(pod, i as u32, j as u32))
        })
        .sum()
}

fn unobstructed(grid: &[Vec<u32>], hall_i: usize, room_i: usize, skip_hall: bool) -> bool {
    (hall_i.min(room_i)..=hall_i.max(room_i)).all(|i| (skip_hall && i == hall_i) || grid[i][0] == 0)
}

fn in_j(grid: &[Vec<u32>], pod: u32) -> Option<usize> {
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

fn out_j(grid: &[Vec<u32>], pod: u32) -> Option<usize> {
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

fn out_moves(grid: &[Vec<u32>], moves: &mut Vec<(Move, u32)>) {
    for &room in ROOMS {
        if let Some(j) = out_j(grid, room as u32) {
            let pod = grid[room][j];
            for &stop in STOPS {
                if unobstructed(grid, stop, room, false) {
                    let m = (room, j, stop, 0);
                    moves.push((m, move_cost(m, pod)));
                }
            }
        }
    }
}

fn in_moves(grid: &[Vec<u32>], moves: &mut Vec<(Move, u32)>) {
    for &stop in STOPS {
        let pod = grid[stop][0];
        if pod != 0 && unobstructed(grid, stop, pod as usize, true) {
            if let Some(j) = in_j(grid, pod) {
                let m = (stop, 0, pod as usize, j);
                moves.push((m, move_cost(m, pod)));
            }
        }
    }
}

fn moves(grid: &[Vec<u32>]) -> Vec<(Move, u32)> {
    let mut moves = vec![];
    out_moves(grid, &mut moves);
    in_moves(grid, &mut moves);
    moves
}

fn apply_move(mut grid: Vec<Vec<u32>>, (i0, j0, i1, j1): Move) -> Vec<Vec<u32>> {
    grid[i1][j1] = grid[i0][j0];
    grid[i0][j0] = 0;
    grid
}

fn solve(grid: Vec<Vec<u32>>) -> u32 {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    heap.push(Reverse((estimate(&grid), 0, grid)));

    while !heap.is_empty() {
        let Reverse((_, cost, grid)) = heap.pop().unwrap();
        if seen.contains(&grid) {
            continue;
        }
        println!();
        print_grid(&grid);
        seen.insert(grid.clone());
        let grid_moves = moves(&grid);
        if solved(&grid) {
            return cost;
        }
        for (m, move_c) in grid_moves {
            let new_grid = apply_move(grid.clone(), m);
            if seen.contains(&new_grid) {
                continue;
            }
            let new_cost = cost + move_c;
            let new_estimate = estimate(&new_grid);
            heap.push(Reverse((new_estimate + new_cost, new_cost, new_grid)));
        }
    }
    unreachable!();
}

fn part1(grid: &[Vec<u32>]) -> u32 {
    solve(grid.to_owned())
}

fn part2(grid: &[Vec<u32>]) -> u32 {
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
            grid[(i + 1) * 2].push((ch as u32 - 'A' as u32 + 1) * 2);
        }
    }
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

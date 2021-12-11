#![feature(drain_filter)]
use std::fs;

type Board = Vec<Vec<i32>>;

fn read_boards(input: String) -> (Vec<i32>, Vec<Board>) {
    let parts = input.split("\n\n").collect::<Vec<_>>();

    let values = parts[0]
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let boards = parts[1..]
        .iter()
        .map(|board_part| {
            board_part
                .lines()
                .map(|board_row| {
                    board_row
                        .split_whitespace()
                        .map(|board_cell| board_cell.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    return (values, boards);
}

fn score_boards(values: Vec<i32>, mut boards: Vec<Board>) -> Vec<i32> {
    let mut success = Vec::new();
    for value in values {
        boards.drain_filter(|board| {
            if let Some(score) = call_num(value, board) {
                success.push(score);
                true
            } else {
                false
            }
        });
    }
    return success;
}

fn sum_unmarked(board: &Board) -> i32 {
    board.iter().flatten().filter(|&&x| x > 0).sum()
}

fn completed(board: &Board, i: usize, j: usize) -> bool {
    board[i].iter().all(|&x| x < 0) || board.iter().map(|row| row[j]).all(|x| x < 0)
}

fn call_num(value: i32, board: &mut Board) -> Option<i32> {
    for i in 0..5 {
        for j in 0..5 {
            if board[i][j] == value {
                board[i][j] = -1;
                return if completed(board, i, j) {
                    Some(value * sum_unmarked(board))
                } else {
                    None
                };
            }
        }
    }
    None
}

fn part1(scores: &Vec<i32>) -> i32 {
    return *scores.first().unwrap();
}

fn part2(scores: &Vec<i32>) -> i32 {
    return *scores.last().unwrap();
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let (values, boards) = read_boards(content);
    let scores = score_boards(values, boards);
    println!("Part 1: {}", part1(&scores));
    println!("Part 2: {}", part2(&scores));
}

use std::fs;

fn part1(data: &Vec<i32>) -> i32 {
    let median = data[data.len() / 2];
    data.iter().map(|&pos| (pos - median).abs()).sum()
}

fn part2(data: &Vec<i32>) -> i32 {
    let mean: i32 = data.iter().sum::<i32>() / data.len() as i32;

    data.iter()
        .map(|&pos| (pos - mean).abs())
        .map(|n| n * (n + 1) / 2)
        .sum()
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let mut data: Vec<i32> = content
        .trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    data.sort();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(data: &[i32]) -> i32 {
    let median = data[data.len() / 2];
    data.iter().map(|&pos| (pos - median).abs()).sum()
}

fn part2(data: &[i32]) -> i32 {
    let mean = data.iter().sum::<i32>() / data.len() as i32;
    data.iter()
        .map(|&pos| (pos - mean).abs())
        .map(|n| n * (n + 1) / 2)
        .sum()
}

fn main() {
    let mut data: Vec<_> = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    data.sort_unstable();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

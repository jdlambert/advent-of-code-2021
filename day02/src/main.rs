fn part1(data: &Vec<Vec<&str>>) -> i64 {
    let (x, y) = data.iter().fold((0, 0), |(x, y), operation| {
        let value: i64 = operation[1].parse().unwrap();
        match operation[0] {
            "forward" => (x + value, y),
            "up" => (x, y - value),
            "down" => (x, y + value),
            _ => unreachable!(),
        }
    });
    x * y
}

fn part2(data: &Vec<Vec<&str>>) -> i64 {
    let (x, y, _) = data.iter().fold((0, 0, 0), |(x, y, aim), operation| {
        let value: i64 = operation[1].parse().unwrap();
        match operation[0] {
            "forward" => (x + value, y + value * aim, aim),
            "up" => (x, y, aim - value),
            "down" => (x, y, aim + value),
            _ => unreachable!(),
        }
    });
    x * y
}

fn main() {
    let data = include_str!("../input.txt")
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

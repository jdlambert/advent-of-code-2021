fn part1(data: &Vec<u32>) -> u32 {
    0
}

fn part2(data: &Vec<u32>) -> u32 {
    0
}

fn main() {
    let data = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

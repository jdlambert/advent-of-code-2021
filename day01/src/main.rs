fn part1(data: &Vec<u32>) -> usize {
    data.windows(2).filter(|x| x.first() < x.last()).count()
}

fn part2(data: &Vec<u32>) -> usize {
    data.windows(4).filter(|x| x.first() < x.last()).count()
}

fn main() {
    let data = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

fn part1(data: &[u32]) -> usize {
    data.windows(2).filter(|x| x.first() < x.last()).count()
}

fn part2(data: &[u32]) -> usize {
    data.windows(4).filter(|x| x.first() < x.last()).count()
}

fn main() {
    let data: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

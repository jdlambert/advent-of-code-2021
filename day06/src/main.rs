fn simulate(data: &Vec<usize>, days: u32) -> u64 {
    let mut counts = vec![0; 9];
    for &fish in data {
        counts[fish] += 1;
    }
    for _ in 0..days {
        counts.rotate_left(1);
        counts[6] += counts[8];
    }
    counts.iter().sum()
}

fn part1(data: &Vec<usize>) -> u64 {
    simulate(data, 80)
}

fn part2(data: &Vec<usize>) -> u64 {
    simulate(data, 256)
}

fn main() {
    let data = include_str!("../input.txt")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

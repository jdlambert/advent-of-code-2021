fn flash(octopi: &mut Vec<Vec<u32>>, i: usize, j: usize) -> u64 {
    let mut flashes = 1;
    octopi[i][j] = 0;
    for i in i.saturating_sub(1)..=(i + 1).min(9) {
        for j in j.saturating_sub(1)..=(j + 1).min(9) {
            if octopi[i][j] == 0 {
                continue;
            }
            octopi[i][j] += 1;
            if octopi[i][j] > 9 {
                flashes += flash(octopi, i, j);
            }
        }
    }
    flashes
}

fn step(octopi: &mut Vec<Vec<u32>>) -> u64 {
    let mut flashes = 0;

    for i in 0..10 {
        for j in 0..10 {
            octopi[i][j] += 1;
        }
    }
    for i in 0..10 {
        for j in 0..10 {
            if octopi[i][j] > 9 {
                flashes += flash(octopi, i, j);
            }
        }
    }
    flashes
}

fn part1(octopi: &Vec<Vec<u32>>) -> u64 {
    let mut octopi = octopi.clone();
    (0..100).map(|_| step(&mut octopi)).sum()
}

fn part2(octopi: &Vec<Vec<u32>>) -> u64 {
    let mut octopi = octopi.clone();
    let mut count = 1;
    loop {
        if step(&mut octopi) == 100 {
            return count;
        }
        count += 1;
    }
}

fn main() {
    let octopi: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect();
    println!("Part 1: {}", part1(&octopi));
    println!("Part 2: {}", part2(&octopi));
}

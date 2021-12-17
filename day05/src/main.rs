use std::collections::HashMap;

type Pair = (i32, i32, i32, i32);

fn dir(a0: i32, a1: i32) -> i32 {
    if a0 > a1 {
        -1
    } else if a0 < a1 {
        1
    } else {
        0
    }
}

fn dist(pair: Pair) -> i32 {
    let (x0, y0, x1, y1) = pair;
    (x0 - x1).abs().max((y0 - y1).abs())
}

fn overlaps(pairs: &[Pair]) -> usize {
    let mut map = HashMap::new();
    for pair in pairs {
        let &(x0, y0, x1, y1) = pair;
        let dx = dir(x0, x1);
        let dy = dir(y0, y1);
        for i in 0..=dist(*pair) {
            *map.entry((x0 + dx * i, y0 + dy * i)).or_insert(0) += 1;
        }
    }
    map.values().filter(|&&val| val > 1).count()
}

fn part1(pairs: &[Pair]) -> usize {
    let straight_lines: Vec<_> = pairs
        .iter()
        .filter(|(x0, y0, x1, y1)| x0 == x1 || y0 == y1)
        .cloned()
        .collect();
    overlaps(&straight_lines)
}

fn part2(pairs: &[Pair]) -> usize {
    overlaps(pairs)
}

fn main() {
    let pairs: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let splits: Vec<i32> = line
                .split(|c: char| !c.is_digit(10))
                .filter_map(|num| num.parse().ok())
                .collect();
            (splits[0], splits[1], splits[2], splits[3])
        })
        .collect();
    println!("Part 1: {}", part1(&pairs));
    println!("Part 2: {}", part2(&pairs));
}

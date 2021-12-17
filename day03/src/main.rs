fn digit_delta(data: &[&str], index: usize) -> i32 {
    data.iter().fold(0, |delta, &line| {
        delta
            + match line.chars().nth(index).unwrap() {
                '1' => 1,
                '0' => -1,
                _ => unreachable!(),
            }
    })
}

fn part1(data: &[&str]) -> u32 {
    let (gamma, epsilon) = (0..data[0].len()).fold((0, 0), |(gamma, epsilon), index| {
        let nth = if digit_delta(data, index) > 0 { 1 } else { 0 };
        ((gamma << 1) | nth, (epsilon << 1) | (1 - nth))
    });
    gamma * epsilon
}

fn life_support_rating(data: &[&str], default: char) -> u32 {
    let mut data = data.to_owned();
    let mut i = 0;
    while data.len() > 1 {
        let nth = if digit_delta(&data, i) >= 0 {
            default
        } else {
            if default == '0' {
                '1'
            } else {
                '0'
            }
        };
        data.retain(|line| line.chars().nth(i).unwrap() == nth);
        i += 1;
    }
    u32::from_str_radix(data[0], 2).unwrap()
}

fn part2(data: &[&str]) -> u32 {
    life_support_rating(data, '1') * life_support_rating(data, '0')
}

fn main() {
    let data: Vec<_> = include_str!("../input.txt").lines().collect();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

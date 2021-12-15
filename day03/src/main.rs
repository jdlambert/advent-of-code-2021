fn digit_delta(data: &Vec<&str>, index: usize) -> i32 {
    let delta = data.iter().fold(0i32, |delta, &line| {
        delta
            + match line.chars().nth(index).unwrap() {
                '1' => 1,
                '0' => -1,
                _ => unreachable!(),
            }
    });
    delta
}

fn part1(data: &Vec<&str>) -> u32 {
    let length = data[0].len();
    let (gamma, epsilon) = (0..length).fold((0, 0), |(gamma, epsilon), index| {
        let nth = if digit_delta(data, index) > 0 { 1 } else { 0 };
        ((gamma << 1) | nth, (epsilon << 1) | (1 - nth))
    });
    gamma * epsilon
}

fn life_support_rating(mut data: Vec<&str>, default: char) -> u32 {
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

fn o2(data: Vec<&str>) -> u32 {
    life_support_rating(data, '1')
}

fn co2(data: Vec<&str>) -> u32 {
    life_support_rating(data, '0')
}

fn part2(data: &Vec<&str>) -> u32 {
    o2(data.clone()) * co2(data.clone())
}

fn main() {
    let data = include_str!("../input.txt").lines().collect();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

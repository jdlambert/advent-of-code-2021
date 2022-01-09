use cached::proc_macro::cached;

static ROLLS: &[(u8, u128)] = &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[cached]
fn universes(this_s: u8, that_s: u8, this_p: u8, that_p: u8) -> (u128, u128) {
    if that_s > 20 {
        return (0, 1);
    }
    let (mut wins, mut losses) = (0, 0);
    for &(roll, freq) in ROLLS {
        let new_p = (this_p + roll - 1) % 10 + 1;
        let (new_losses, new_wins) = universes(that_s, this_s + new_p, that_p, new_p);
        wins += new_wins * freq;
        losses += new_losses * freq;
    }
    (wins, losses)
}

fn part2(one: u32, two: u32) -> u128 {
    let (one, two) = universes(0, 0, one as u8, two as u8);
    one.max(two)
}

fn play(
    mut this_s: u32,
    mut that_s: u32,
    mut this_p: u32,
    mut that_p: u32,
    mut die: u32,
    mut rolls: u32,
) -> (u32, u32) {
    loop {
        for _ in 0..3 {
            this_p = (this_p + die - 1) % 10 + 1;
            die = die % 100 + 1;
        }
        rolls += 3;
        this_s += this_p;
        if this_s > 999 {
            return (that_s, rolls);
        }
        std::mem::swap(&mut this_s, &mut that_s);
        std::mem::swap(&mut this_p, &mut that_p);
    }
}

fn part1(one: u32, two: u32) -> u32 {
    let (lower_score, rolls) = play(0, 0, one, two, 1, 0);
    lower_score * rolls
}

fn parse(line: &str) -> u32 {
    line.trim()
        .chars()
        .last()
        .and_then(|c| c.to_digit(10))
        .unwrap()
}

fn main() {
    let (one, two) = include_str!("../input.txt").split_once("\n").unwrap();
    let one = parse(one);
    let two = parse(two);
    println!("Part 1: {}", part1(one, two));
    println!("Part 2: {}", part2(one, two));
}

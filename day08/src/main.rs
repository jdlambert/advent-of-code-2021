fn part1(notes: &[Vec<Vec<String>>]) -> usize {
    notes
        .iter()
        .map(|note| &note[1])
        .flatten()
        .filter(|output| match output.len() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        })
        .count()
}

fn len_n_symbols(input: &[String], n: usize) -> Vec<&str> {
    input
        .iter()
        .filter(|s| s.len() == n)
        .map(|s| s.as_ref())
        .collect()
}

fn len_n_symbol(input: &[String], n: usize) -> &str {
    let symbols = len_n_symbols(input, n);
    assert_eq!(symbols.len(), 1);
    symbols[0]
}

fn contains(container: &str, containee: &str) -> bool {
    containee
        .chars()
        .all(|letter| container.chars().any(|c| c == letter))
}

fn find_symbol<'a>(candidates: &'a [&str], predicate: impl Fn(&&&str) -> bool) -> &'a str {
    candidates.iter().find(predicate).unwrap()
}

fn solve_note(note: &Vec<Vec<String>>) -> u32 {
    let input = &note[0];
    let output = &note[1];
    let mut signals = vec![""; 10];

    signals[8] = "abcdefg"; // free symbol
    signals[1] = len_n_symbol(input, 2); // cf
    signals[7] = len_n_symbol(input, 3); // acf
    signals[4] = len_n_symbol(input, 4); // bcdf

    let zero_six_nine = len_n_symbols(input, 6); // abcefg, abdefg, abcdfg
    signals[6] = find_symbol(&zero_six_nine, |symbol| !contains(symbol, signals[1]));
    signals[9] = find_symbol(&zero_six_nine, |symbol| contains(symbol, signals[4]));
    signals[0] = find_symbol(&zero_six_nine, |&&symbol| {
        symbol != signals[6] && symbol != signals[9]
    });

    let two_three_five = len_n_symbols(input, 5); // acdeg, acdfg, abdfg
    signals[3] = find_symbol(&two_three_five, |symbol| contains(symbol, signals[1]));
    signals[5] = find_symbol(&two_three_five, |symbol| contains(signals[6], symbol));
    signals[2] = find_symbol(&two_three_five, |&&symbol| {
        symbol != signals[3] && symbol != signals[5]
    });

    output.iter().fold(0, |total, next| {
        total * 10 + signals.iter().position(|signal| signal == next).unwrap() as u32
    })
}

fn part2(data: &[Vec<Vec<String>>]) -> u32 {
    data.iter().map(solve_note).sum()
}

fn parse_note(note: &str) -> Vec<String> {
    note.trim()
        .split_whitespace()
        .map(|s| {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            String::from_iter(chars)
        })
        .collect()
}

fn main() {
    let data: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|line| line.split('|').map(parse_note).collect())
        .collect();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

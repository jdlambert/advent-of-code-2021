use std::collections::HashMap;
use std::fs;

fn part1(notes: &Vec<Vec<Vec<String>>>) -> usize {
    notes
        .iter()
        .map(|note| &note[1])
        .flatten()
        .filter(|output| match output.len() {
            2 => true,
            3 => true,
            4 => true,
            7 => true,
            _ => false,
        })
        .count()
}

fn len_n_symbols(input: &Vec<String>, n: usize) -> Vec<&str> {
    input
        .iter()
        .filter(|s| s.len() == n)
        .map(|s| s.as_ref())
        .collect()
}

fn len_n_symbol(input: &Vec<String>, n: usize) -> &str {
    let symbols = len_n_symbols(input, n);
    assert_eq!(symbols.len(), 1);
    symbols[0]
}

fn contains(container: &str, containee: &str) -> bool {
    containee
        .chars()
        .all(|letter| container.chars().find(|&c| c == letter).is_some())
}

fn difference(a: &str, b: &str) -> String {
    a.chars()
        .filter(|letter| b.chars().find(|c| c == letter).is_none())
        .collect()
}

fn solve_note(note: &Vec<Vec<String>>) -> u32 {
    let input = &note[0];
    let output = &note[1];
    let mut symbol_table: HashMap<&str, char> = HashMap::new();

    symbol_table.insert("abcdefg", '8'); // free symbol

    let one = len_n_symbol(input, 2); // cf
    symbol_table.insert(one, '1');

    let seven = len_n_symbol(input, 3); // acf
    symbol_table.insert(seven, '7');

    let four = len_n_symbol(input, 4); // bcdf
    symbol_table.insert(four, '4');

    let two_three_five = len_n_symbols(input, 5); // acdeg, acdfg, abdfg

    let three = two_three_five
        .iter()
        .find(|symbol| contains(symbol, one))
        .unwrap();

    let five = two_three_five
        .iter()
        .find(|symbol| contains(symbol, &difference(four, one)))
        .unwrap();

    let two = two_three_five
        .iter()
        .find(|&s| s != three && s != five)
        .unwrap();

    symbol_table.insert(two, '2');
    symbol_table.insert(three, '3');
    symbol_table.insert(five, '5');

    let zero_six_nine = len_n_symbols(input, 6); // abcefg, abdefg, abcdfg
    let six = zero_six_nine
        .iter()
        .find(|&symbol| !contains(symbol, one))
        .unwrap();
    let nine = zero_six_nine
        .iter()
        .find(|&symbol| contains(symbol, four))
        .unwrap();
    let zero = zero_six_nine
        .iter()
        .find(|&symbol| symbol != six && symbol != nine)
        .unwrap();
    symbol_table.insert(zero, '0');
    symbol_table.insert(six, '6');
    symbol_table.insert(nine, '9');

    let digits: String = output
        .iter()
        .map(|symbol| symbol_table[symbol.as_str()])
        .collect();

    u32::from_str_radix(&digits, 10).unwrap()
}

fn part2(data: &Vec<Vec<Vec<String>>>) -> u32 {
    data.iter().map(solve_note).sum()
}

fn parse_note(note: &str) -> Vec<String> {
    note.trim()
        .split_whitespace()
        .map(|s| {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            String::from_iter(chars)
        })
        .collect()
}

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let lines = content.lines();
    let data: Vec<Vec<Vec<String>>> = lines
        .map(|line| line.split('|').map(parse_note).collect())
        .collect();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

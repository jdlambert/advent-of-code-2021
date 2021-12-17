enum LineResult {
    Illegal(char),
    Incomplete(Vec<char>),
}

fn line_eval(line: &Vec<char>) -> LineResult {
    let mut stack = Vec::new();
    for &ch in line {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                let expected = match ch {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    '>' => '<',
                    _ => unreachable!(),
                };
                if stack.pop().unwrap() != expected {
                    return LineResult::Illegal(ch);
                }
            }
            _ => unreachable!(),
        }
    }
    LineResult::Incomplete(
        stack
            .iter()
            .rev()
            .map(|ch| match ch {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => unreachable!(),
            })
            .collect(),
    )
}

fn part1(data: &Vec<Vec<char>>) -> u32 {
    data.iter()
        .map(|line| {
            if let LineResult::Illegal(c) = line_eval(line) {
                match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                }
            } else {
                0
            }
        })
        .sum()
}

fn part2(data: &Vec<Vec<char>>) -> u64 {
    let mut scores: Vec<u64> = data
        .iter()
        .filter_map(|line| {
            if let LineResult::Incomplete(v) = line_eval(line) {
                Some(v.iter().fold(0, |total, ch| {
                    total * 5
                        + match ch {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => unreachable!(),
                        }
                }))
            } else {
                None
            }
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

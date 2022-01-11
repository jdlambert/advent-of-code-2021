type Block = (i8, i8, i8);
type Program = Vec<Block>;

fn solve(program: &Program, init: i8) -> u64 {
    let mut num = vec![init; 14];
    let mut stack = vec![];
    for (i, &(div, check, _)) in program.iter().enumerate() {
        if div == 1 {
            stack.push(i);
        } else {
            let j = stack.pop().unwrap();
            num[i] = num[j] + program[j].2 + check;
            while num[i] > 9 {
                num[i] -= 1;
                num[j] -= 1;
            }
            while num[i] < 1 {
                num[i] += 1;
                num[j] += 1;
            }
        }
    }
    num.iter().fold(0, |result, &next| result * 10 + next as u64)
}

fn part2(program: &Program) -> u64 {
    solve(program, 1)
}

fn part1(program: &Program) -> u64 {
    solve(program, 9)
}

fn extract(s: &str) -> i8 {
    let num: String = s.chars().filter(|&c| c.is_digit(10) || c == '-').collect();
    i8::from_str_radix(&num, 10).unwrap()
}

fn main() {
    let lines: Vec<_> = include_str!("../input.txt").lines().collect();
    let program: Vec<_> = lines.chunks_exact(18).map(|block_lines| {
        (extract(block_lines[4]), extract(block_lines[5]), extract(block_lines[15]))
    }).collect();
    println!("{}", part1(&program));
    println!("{}", part2(&program));
}
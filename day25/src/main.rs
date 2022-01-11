#[derive(PartialEq, Clone, Copy)]
enum Cuke {
    E,
    S,
}

type Cukes = Vec<Vec<Option<Cuke>>>;

fn should_move(cukes: &Cukes, i: usize, j: usize, allowed: Cuke) -> bool {
    match &cukes[i][j] {
        Some(cuke) if *cuke == allowed => {
            let (i, j) = match allowed {
                Cuke::E => (i, (j + 1) % cukes[0].len()),
                Cuke::S => ((i + 1) % cukes.len(), j),
            };
            cukes[i][j].is_none()
        }
        _ => false,
    }
}

fn do_move(cukes: &mut Cukes, i: usize, j: usize) {
    let h = cukes.len();
    let w = cukes[0].len();
    match &cukes[i][j] {
        Some(Cuke::E) => {
            cukes[i][(j + 1) % w] = Some(Cuke::E);
            cukes[i][j] = None;
        }
        Some(Cuke::S) => {
            cukes[(i + 1) % h][j] = Some(Cuke::S);
            cukes[i][j] = None;
        }
        _ => unreachable!(),
    }
}

fn perform_moves(cukes: &mut Cukes, allowed: Cuke) -> bool {
    let mut moves = vec![];
    for i in 0..cukes.len() {
        for j in 0..cukes[0].len() {
            if should_move(cukes, i, j, allowed) {
                moves.push((i, j))
            }
        }
    }
    let moved = !moves.is_empty();
    for &(i, j) in &moves {
        do_move(cukes, i, j)
    }
    moved
}

fn part1(mut cukes: Cukes) -> u32 {
    let mut step = 0;
    loop {
        step += 1;
        let moved = perform_moves(&mut cukes, Cuke::E);
        if !perform_moves(&mut cukes, Cuke::S) && !moved {
            return step;
        }
    }
}

fn main() {
    let cukes: Vec<_> = include_str!("../input.txt")
        .trim()
        .lines()
        .map(|x| {
            x.chars()
                .map(move |c| match c {
                    '>' => Some(Cuke::E),
                    'v' => Some(Cuke::S),
                    _ => None,
                })
                .collect()
        })
        .collect();
    println!("Part 1: {}", part1(cukes));
}

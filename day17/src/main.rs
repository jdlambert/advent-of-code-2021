const X: (isize, isize) = (287, 309);
const Y: (isize, isize) = (-76, -48);

fn shoot(mut dx: isize, mut dy: isize) -> Option<isize> {
    let mut max_y = isize::MIN;
    let mut x = 0;
    let mut y = 0;

    while x <= X.1 && y >= Y.0 {
        x += dx;
        y += dy;
        max_y = max_y.max(y);
        dx -= dx.signum();
        dy -= 1;
        if X.0 <= x && x <= X.1 && Y.0 <= y && y <= Y.1 {
            return Some(max_y);
        }
    }
    None
}

fn main() {
    let high_points: Vec<_> = (0..=X.1)
        .flat_map(|dx| (Y.0..=-Y.0).map(move |dy| (dx, dy)))
        .filter_map(|(dx, dy)| shoot(dx, dy))
        .collect();
    println!("Part 1: {}", high_points.iter().max().unwrap());
    println!("Part 2: {}", high_points.len());
}

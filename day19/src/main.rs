use std::collections::{HashMap, HashSet};

type Point = (isize, isize, isize);
type Fingerprint = (isize, isize);
type Fingerprints = HashMap<Fingerprint, Vec<(Point, Point)>>;

fn sub((x0, y0, z0): Point, (x1, y1, z1): Point) -> Point {
    (x0 - x1, y0 - y1, z0 - z1)
}

fn add((x0, y0, z0): Point, (x1, y1, z1): Point) -> Point {
    (x0 + x1, y0 + y1, z0 + z1)
}

fn l1_diff(a: Point, b: Point) -> isize {
    let (dx, dy, dz) = sub(a, b);
    dx.abs() + dy.abs() + dz.abs()
}

fn lmax_diff(a: Point, b: Point) -> isize {
    let (dx, dy, dz) = sub(a, b);
    dx.abs().max(dy.abs()).max(dz.abs())
}

fn fingerprint(a: Point, b: Point) -> Fingerprint {
    (l1_diff(a, b), lmax_diff(a, b))
}

fn part1((_, beacons): &(Vec<Point>, HashSet<Point>)) -> usize {
    beacons.len()
}

fn part2((scanners, _): &(Vec<Point>, HashSet<Point>)) -> isize {
    scanners
        .iter()
        .flat_map(|l| scanners.iter().map(move |r| (l, r)))
        .map(|(&a, &b)| l1_diff(a, b))
        .max()
        .unwrap()
}

fn rotate((x, y, z): Point, i: usize) -> Point {
    let (x, y, z) = match i / 4 {
        0 => (x, y, z),
        1 => (-x, y, -z),
        2 => (y, x, -z),
        3 => (-y, x, z),
        4 => (z, x, y),
        5 => (-z, x, -y),
        _ => unreachable!(),
    };
    match i % 4 {
        0 => (x, y, z),
        1 => (x, -y, -z),
        2 => (x, z, -y),
        3 => (x, -z, y),
        _ => unreachable!(),
    }
}

fn find_match(
    known_fingerprints: &Fingerprints,
    beacons: &[Point],
    fingerprints: &[(Fingerprint, (Point, Point))],
) -> Option<(Point, Vec<Point>)> {
    let matching_fprints: Vec<_> = fingerprints
        .iter()
        .filter(|(fprint, _)| known_fingerprints.contains_key(fprint))
        .collect();

    if matching_fprints.len() < 66 {
        return None;
    }

    for (f, unknown_pair) in matching_fprints {
        for known_pair in known_fingerprints.get(f).unwrap() {
            let (k0, k1) = *known_pair;
            let (u0, u1) = *unknown_pair;

            for rotation in 0..24 {
                if sub(k0, rotate(u0, rotation)) == sub(k1, rotate(u1, rotation)) {
                    let translation = sub(k0, rotate(u0, rotation));

                    let transformed_beacons: Vec<_> = beacons
                        .iter()
                        .map(|&p| add(translation, rotate(p, rotation)))
                        .collect();
                    return Some((translation, transformed_beacons));
                }
            }
        }
    }
    unreachable!();
}

fn extend_fingerprints(fingerprints: &mut Fingerprints, scanner: &[Point]) {
    for i in 0..scanner.len() {
        for j in (i + 1)..scanner.len() {
            let f = fingerprint(scanner[i], scanner[j]);
            fingerprints
                .entry(f)
                .or_default()
                .push((scanner[i], scanner[j]));
        }
    }
}

fn solve(scanners: &[Vec<Point>]) -> (Vec<Point>, HashSet<Point>) {
    let mut known_scanners = vec![(0, 0, 0)];
    let mut known_beacons: HashSet<_> = scanners[0].clone().into_iter().collect();
    let mut known_fingerprints = HashMap::new();
    extend_fingerprints(&mut known_fingerprints, &scanners[0]);

    let mut unknown_beacons: Vec<_> = scanners[1..]
        .iter()
        .map(|scanner| {
            let fingerprints: Vec<_> = scanner
                .iter()
                .flat_map(|&a| scanner.iter().map(move |&b| (fingerprint(a, b), (a, b))))
                .collect();
            (scanner, fingerprints)
        })
        .collect();

    while !unknown_beacons.is_empty() {
        unknown_beacons.retain(|(beacons, fingerprints)| {
            if let Some((scanner, transformed_beacons)) =
                find_match(&known_fingerprints, beacons, fingerprints)
            {
                extend_fingerprints(&mut known_fingerprints, &transformed_beacons);
                known_scanners.push(scanner);
                known_beacons.extend(transformed_beacons);
                false
            } else {
                true
            }
        });
    }
    (known_scanners, known_beacons)
}

fn main() {
    let scanners: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    let nums: Vec<_> = line.split(',').map(|num| num.parse().unwrap()).collect();
                    (nums[0], nums[1], nums[2])
                })
                .collect()
        })
        .collect();
    let solution = solve(&scanners);
    println!("Part 1: {}", part1(&solution));
    println!("Part 2: {}", part2(&solution));
}

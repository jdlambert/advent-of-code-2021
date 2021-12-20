use std::collections::{HashMap, HashSet};

type Point = (isize, isize, isize);

fn sub((x0, y0, z0): Point, (x1, y1, z1): Point) -> Point {
    (x0 - x1, y0 - y1, z0 - z1)
}

fn add((x0, y0, z0): Point, (x1, y1, z1): Point) -> Point {
    (x0 + x1, y0 + y1, z0 + z1)
}

fn norm(a: Point, b: Point) -> isize {
    let (dx, dy, dz) = sub(a, b);
    dx * dx + dy * dy + dz * dz
}

fn part1(solution: &[(Point, Vec<Point>)]) -> usize {
    solution
        .iter()
        .flat_map(|(_, beacons)| beacons)
        .collect::<HashSet<_>>()
        .len()
}

fn part2(solution: &[(Point, Vec<Point>)]) -> isize {
    solution
        .iter()
        .flat_map(|(l, _)| solution.iter().map(move |(r, _)| (l, r)))
        .map(|((x0, y0, z0), (x1, y1, z1))| (x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs())
        .max()
        .unwrap()
}

fn orient((x, y, z): Point, i: usize) -> Point {
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

fn rectify(
    unsolved_beacon: Point,
    solved_beacon: Point,
    unsolved_beacons: &[Point],
    solved_beacons: &[Point],
    unsolved_norms: &HashMap<Point, HashSet<isize>>,
) -> (Point, (Vec<Point>, HashMap<Point, HashSet<isize>>)) {
    let solved_set: HashSet<_> = solved_beacons.iter().collect();
    for orientation in 0..24 {
        let oriented_unsolved_beacon = orient(unsolved_beacon, orientation);
        let displacement = sub(solved_beacon, oriented_unsolved_beacon);
        let corrected_beacons: Vec<_> = unsolved_beacons
            .iter()
            .map(|&beacon| add(displacement, orient(beacon, orientation)))
            .collect();
        let corrected_set: HashSet<_> = corrected_beacons.iter().collect();
        let overlap: HashSet<_> = corrected_set.intersection(&solved_set).collect();
        if overlap.len() > 11 {
            let corrected_norms: HashMap<_, _> = unsolved_norms
                .iter()
                .map(|(point, norms)| {
                    (
                        add(displacement, orient(*point, orientation)),
                        norms.clone(),
                    )
                })
                .collect();
            return (
                sub((0, 0, 0), displacement),
                (corrected_beacons, corrected_norms),
            );
        }
    }
    unreachable!()
}

fn solve(scanners: &[Vec<Point>]) -> Vec<(Point, Vec<Point>)> {
    let displacements: Vec<_> = scanners
        .iter()
        .map(|beacons| {
            let mut pairs: HashMap<_, HashSet<_>> = HashMap::new();
            for i in 0..beacons.len() {
                for j in (i + 1)..beacons.len() {
                    let n = norm(beacons[i], beacons[j]);
                    pairs.entry(beacons[i]).or_default().insert(n);
                    pairs.entry(beacons[j]).or_default().insert(n);
                }
            }
            pairs
        })
        .collect();

    let zipped: Vec<_> = scanners.iter().zip(displacements).collect();

    let mut solved = vec![((0, 0, 0), (zipped[0].0.clone(), zipped[0].1.clone()))];
    let mut unsolved: Vec<_> = zipped[1..].iter().collect();

    while !unsolved.is_empty() {
        unsolved.retain(|(unsolved_beacons, unsolved_displacements)| {
            for (_, (solved_beacons, solved_displacements)) in &solved {
                for unsolved_beacon in *unsolved_beacons {
                    for solved_beacon in solved_beacons {
                        let unsolved_norms = unsolved_displacements.get(unsolved_beacon).unwrap();
                        let solved_norms = solved_displacements.get(solved_beacon).unwrap();
                        let intersect: HashSet<_> =
                            unsolved_norms.intersection(solved_norms).collect();
                        if intersect.len() > 10 {
                            let rectified = rectify(
                                *unsolved_beacon,
                                *solved_beacon,
                                unsolved_beacons,
                                solved_beacons,
                                unsolved_displacements,
                            );
                            solved.push(rectified);
                            return false;
                        }
                    }
                }
            }
            true
        });
    }

    solved
        .into_iter()
        .map(|(origin, (beacons, _))| (origin, beacons))
        .collect()
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

#[derive(Clone, Copy, Debug)]
struct Instruction {
    on: bool,
    region: Region,
}

type Interval = (isize, isize);

fn len(interval: Interval) -> isize {
    interval.1 - interval.0 + 1
}

fn contains(interval: Interval, point: isize) -> bool {
    interval.0 <= point && point <= interval.1
}

fn intersects(this: Interval, that: Interval) -> bool {
    contains(this, that.0)
        || contains(this, that.1)
        || contains(that, this.0)
        || contains(that, this.1)
}

fn clamp(a: Interval, b: Interval) -> Interval {
    (a.0.max(b.0), a.1.min(b.1))
}

#[derive(Clone, Copy, Debug)]
struct Region {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Region {
    fn intersects(&self, other: &Self) -> bool {
        intersects(self.x, other.x) && intersects(self.y, other.y) && intersects(self.z, other.z)
    }

    fn volume(&self) -> isize {
        len(self.x) * len(self.y) * len(self.z)
    }

    fn diff(&self, other: &Self) -> Vec<Self> {
        if !self.intersects(other) {
            return vec![*self];
        }
        let Region { x, y, z } = *self;
        let mut result = vec![];
        if x.0 < other.x.0 {
            result.push(Self {
                x: (x.0, other.x.0 - 1),
                y,
                z,
            });
        }
        if self.x.1 > other.x.1 {
            result.push(Self {
                x: (other.x.1 + 1, x.1),
                y,
                z,
            });
        }

        let clamp_x = clamp(x, other.x);
        if y.0 < other.y.0 {
            result.push(Self {
                x: clamp_x,
                y: (y.0, other.y.0 - 1),
                z,
            });
        }
        if other.y.1 < y.1 {
            result.push(Self {
                x: clamp_x,
                y: (other.y.1 + 1, y.1),
                z,
            });
        }

        let clamp_y = clamp(y, other.y);
        if z.0 < other.z.0 {
            result.push(Self {
                x: clamp_x,
                y: clamp_y,
                z: (z.0, other.z.0 - 1),
            });
        }
        if other.z.1 < z.1 {
            result.push(Self {
                x: clamp_x,
                y: clamp_y,
                z: (other.z.1 + 1, z.1),
            });
        }

        result
    }
}

fn insert_region(mut regions: Vec<Region>, region: Region, on: bool) -> Vec<Region> {
    if on {
        let mut new_regions = vec![region];
        for old_region in &regions {
            new_regions = insert_region(new_regions, *old_region, false);
        }
        regions.extend(new_regions);
        regions
    } else {
        regions
            .iter()
            .flat_map(|old_region| old_region.diff(&region))
            .collect()
    }
}

fn run(instructions: &Vec<Instruction>) -> isize {
    let mut regions = vec![];
    for &Instruction { on, region } in instructions {
        regions = insert_region(regions, region, on);
    }
    regions.iter().map(Region::volume).sum()
}

fn part1(instructions: &Vec<Instruction>) -> isize {
    let instructions = instructions
        .into_iter()
        .take_while(|Instruction { region, .. }| region.x.0 >= -50 && region.x.1 <= 50)
        .cloned()
        .collect();

    run(&instructions)
}

fn part2(instructions: &Vec<Instruction>) -> isize {
    run(instructions)
}

fn main() {
    let instructions: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (switch, region) = line.split_once(' ').unwrap();
            let on = switch == "on";
            let coords: Vec<_> = region
                .split(',')
                .flat_map(|coord| {
                    let (start, end) = coord[2..].split_once("..").unwrap();
                    [start.parse().unwrap(), end.parse().unwrap()]
                })
                .collect();
            let region = Region {
                x: (coords[0], coords[1]),
                y: (coords[2], coords[3]),
                z: (coords[4], coords[5]),
            };
            Instruction { on, region }
        })
        .collect();
    println!("Part 1: {}", part1(&instructions));
    println!("Part 1: {}", part2(&instructions));
}

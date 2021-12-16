use std::collections::HashSet;

fn part1(heights: &Vec<Vec<u32>>) -> u32 {
    let mut risk = 0;
    for (i, row) in heights.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if (i == 0 || heights[i - 1][j] > height)
                && (i == heights.len() - 1 || heights[i + 1][j] > height)
                && (j == 0 || heights[i][j - 1] > height)
                && (j == heights[0].len() - 1 || heights[i][j + 1] > height)
            {
                risk += height + 1
            }
        }
    }
    risk
}

fn basin_size(heights: &Vec<Vec<u32>>, i: usize, j: usize) -> usize {
    let mut frontier = vec![(i, j)];
    let mut seen = HashSet::new();

    while !frontier.is_empty() {
        let (x, y) = frontier.pop().unwrap();
        if seen.contains(&(x, y)) || heights[x][y] == 9 {
            continue;
        }
        seen.insert((x, y));
        let height = heights[x][y];

        if x != 0 && heights[x - 1][y] > height {
            frontier.push((x - 1, y));
        }
        if x != heights.len() - 1 && heights[x + 1][y] > height {
            frontier.push((x + 1, y));
        }
        if y != 0 && heights[x][y - 1] > height {
            frontier.push((x, y - 1));
        }
        if y != heights[0].len() - 1 && heights[x][y + 1] > height {
            frontier.push((x, y + 1));
        }
    }

    seen.len()
}

fn part2(heights: &Vec<Vec<u32>>) -> usize {
    let mut basin_sizes = vec![];
    for (i, row) in heights.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            if (i == 0 || heights[i - 1][j] > height)
                && (i == heights.len() - 1 || heights[i + 1][j] > height)
                && (j == 0 || heights[i][j - 1] > height)
                && (j == heights[0].len() - 1 || heights[i][j + 1] > height)
            {
                basin_sizes.push(basin_size(heights, i, j));
            }
        }
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes.iter().take(3).product()
}

fn main() {
    let heights = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|num| num.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    println!("Part 1: {}", part1(&heights));
    println!("Part 2: {}", part2(&heights));
}

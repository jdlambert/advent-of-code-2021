use std::collections::{HashMap, HashSet};

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn count_paths(graph: &Graph, small_cave_exception: bool) -> u32 {
    let mut blocked = HashSet::new();
    count_paths_helper(graph, "start", &mut blocked, small_cave_exception)
}

fn count_paths_helper<'a>(
    graph: &Graph<'a>,
    current: &str,
    blocked: &mut HashSet<&'a str>,
    small_cave_exception: bool,
) -> u32 {
    graph[current]
        .iter()
        .map(|neighbor| match *neighbor {
            "end" => 1,
            "start" => 0,
            _ => {
                if blocked.contains(neighbor) && !small_cave_exception {
                    return 0;
                }
                let used_exception = blocked.contains(neighbor) && small_cave_exception;
                if !neighbor.chars().next().unwrap().is_ascii_uppercase() {
                    blocked.insert(neighbor);
                }
                let count = count_paths_helper(
                    graph,
                    neighbor,
                    blocked,
                    small_cave_exception && !used_exception,
                );
                if !used_exception {
                    blocked.remove(neighbor);
                }
                count
            }
        })
        .sum()
}

fn part1(graph: &Graph) -> u32 {
    count_paths(graph, false)
}

fn part2(graph: &Graph) -> u32 {
    count_paths(graph, true)
}

fn main() {
    let mut graph: Graph = HashMap::new();
    for line in include_str!("../input.txt").lines() {
        let mut splits = line.split('-');
        let l = splits.next().unwrap();
        let r = splits.next().unwrap();

        graph.entry(l).or_default().push(r);
        graph.entry(r).or_default().push(l);
    }
    println!("Part 1: {}", part1(&graph));
    println!("Part 2: {}", part2(&graph));
}

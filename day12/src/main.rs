use std::collections::{HashMap, HashSet};
use std::fs;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn count_paths(graph: &Graph, small_cave_exception: bool) -> u32 {
    let mut path = vec!["start"];
    let mut blocked = HashSet::new();
    count_paths_helper(graph, &mut path, &mut blocked, small_cave_exception)
}

fn count_paths_helper<'a>(
    graph: &Graph<'a>,
    path: &mut Vec<&'a str>,
    blocked: &mut HashSet<&'a str>,
    small_cave_exception: bool,
) -> u32 {
    graph[path.last().unwrap()]
        .iter()
        .filter_map(|neighbor| match *neighbor {
            "end" => Some(1),
            "start" => None,
            _ => {
                if blocked.contains(neighbor) && !small_cave_exception {
                    return None;
                }
                let used_exception = blocked.contains(neighbor) && small_cave_exception;
                path.push(neighbor);
                if !neighbor.chars().next().unwrap().is_ascii_uppercase() {
                    blocked.insert(neighbor);
                }
                let count = count_paths_helper(
                    graph,
                    path,
                    blocked,
                    small_cave_exception && !used_exception,
                );
                if !used_exception {
                    blocked.remove(neighbor);
                }
                path.pop();
                Some(count)
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
    let content = fs::read_to_string("./input.txt").unwrap();
    let mut graph: Graph = HashMap::new();
    for line in content.lines() {
        let mut splits = line.split('-');
        let l = splits.next().unwrap();
        let r = splits.next().unwrap();

        graph.entry(l).or_default().push(r);
        graph.entry(r).or_default().push(l);
    }
    println!("Part 1: {}", part1(&graph));
    println!("Part 2: {}", part2(&graph));
}

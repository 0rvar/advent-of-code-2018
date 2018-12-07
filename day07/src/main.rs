use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<(char, char)> = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|x| {
            let parts = x
                .split(" ")
                .filter(|x| x.len() == 1)
                .map(|x| x.chars().next().unwrap())
                .collect::<Vec<char>>();

            (parts[0], parts[1])
        })
        .collect();

    let mut nodes = HashSet::new();
    let mut edges: HashMap<char, Vec<char>> = HashMap::new();
    for (x, y) in input {
        nodes.insert(x);
        nodes.insert(y);

        edges.entry(x).or_insert_with(|| vec![]).push(y);
    }

    let mut visited_nodes: Vec<char> = Vec::new();
    loop {
        let mut node_candidates = nodes.clone();
        for (parent, children) in &edges {
            if visited_nodes.contains(&parent) {
                node_candidates.remove(&parent);
                continue;
            }
            for node in children {
                node_candidates.remove(&node);
            }
        }

        let node_candidates = node_candidates
            .iter()
            .filter(|x| !visited_nodes.contains(x))
            .map(|x| *x)
            .collect::<Vec<char>>();
        if node_candidates.len() < 1 {
            break;
        }

        visited_nodes.push(*node_candidates.iter().min().unwrap());
    }

    println!("Part 1: {:?}", visited_nodes.iter().collect::<String>());
}

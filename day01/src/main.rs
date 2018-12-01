use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let changes: Vec<isize> = input
        .trim()
        .split("\n")
        .map(|x| parse(x))
        .collect::<Vec<_>>();

    println!("Part 1: {}", changes.iter().sum::<isize>());

    let mut visited_frequencies = HashSet::new();
    visited_frequencies.insert(0isize);
    let mut current_frequency = 0isize;
    for delta in changes.iter().cycle() {
        current_frequency += delta;
        if visited_frequencies.contains(&current_frequency) {
            break;
        }
        visited_frequencies.insert(current_frequency);
    }
    println!("Part 2: {}", current_frequency);
}

fn parse(line: &str) -> isize {
    line.parse().unwrap()
}

#[test]
fn test_parse() {
    assert_eq!(parse("+10"), 10isize);
    assert_eq!(parse("-10"), -10isize);
}

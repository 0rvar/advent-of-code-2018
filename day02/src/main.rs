use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let sizes: Vec<&str> = input.trim().split("\n").collect::<Vec<_>>();

    let part_1_checksum: usize = sizes.iter().map(|x| hash(x, 2)).sum::<usize>()
        * sizes.iter().map(|x| hash(x, 3)).sum::<usize>();
    println!("Part 1: {}", part_1_checksum);
}

fn hash(s: &str, count: usize) -> usize {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }
    if counts.values().any(|&x| x == count) {
        1
    } else {
        0
    }
}

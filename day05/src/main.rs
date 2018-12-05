use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt").trim();
    // println!("Part 1: {}", polymer_reaction(input).len());

    for unit_to_remove in find_units(input) {
        let filtered = input
            .chars()
            .filter(|&x| x != unit_to_remove)
            .collect::<String>();
        println!(
            "Part 2: removing {} gives len {}",
            unit_to_remove,
            polymer_reaction(&filtered).len()
        );
    }
}

fn find_units(polymer: &str) -> Vec<char> {
    let mut pairs: HashSet<char> = HashSet::new();
    for x in polymer.chars() {
        pairs.insert(x.to_ascii_lowercase());
    }

    pairs.iter().map(|x| *x).collect::<Vec<char>>()
}

fn polymer_reaction(polymer: &str) -> String {
    let polymer_len = polymer.len();
    if polymer_len < 1 {
        return "".to_string();
    }
    let mut removed_indices = HashSet::new();
    let mut char_lookup = HashMap::new();
    for (i, x) in polymer.chars().enumerate() {
        char_lookup.insert(i, x);
    }
    'outer: loop {
        'scan: for i in 0..(polymer_len - 1) {
            if removed_indices.contains(&i) {
                continue;
            }
            let mut j: usize = i + 1;
            for index in (i + 1)..polymer_len {
                if !removed_indices.contains(&index) {
                    j = index;
                    break;
                }
                if index == polymer_len - 1 {
                    continue 'scan;
                }
            }
            let this = char_lookup.get(&i).unwrap();
            let next = char_lookup.get(&j).unwrap();
            if units_will_react(*this, *next) {
                removed_indices.insert(i);
                removed_indices.insert(j);
                continue 'outer;
            }
        }
        break 'outer;
    }

    polymer
        .chars()
        .enumerate()
        .filter(|(index, _)| !removed_indices.contains(index))
        .map(|(_, x)| x)
        .collect::<String>()
}

#[test]
fn test_polymer_reaction() {
    assert_eq!(polymer_reaction("Aa"), "".to_string());
    assert_eq!(polymer_reaction("aA"), "".to_string());
    assert_eq!(polymer_reaction("abBA"), "".to_string());
    assert_eq!(polymer_reaction("aabAAB"), "aabAAB".to_string());
}

fn units_will_react(a: char, b: char) -> bool {
    return (a.to_ascii_lowercase() == a
        && b.to_ascii_uppercase() == b
        && a.to_ascii_uppercase() == b)
        || (a.to_ascii_uppercase() == a
            && b.to_ascii_lowercase() == b
            && a.to_ascii_lowercase() == b);
}

#[test]
fn test_units_will_react() {
    assert_eq!(units_will_react('a', 'A'), true);
    assert_eq!(units_will_react('A', 'a'), true);
    assert_eq!(units_will_react('a', 'b'), false);
    assert_eq!(units_will_react('a', 'B'), false);
}

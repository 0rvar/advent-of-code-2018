use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .bytes()
        .collect::<Vec<u8>>();
    println!("Part 1: {}", polymer_reaction(&input).len());

    let lengths = find_units(&input)
        .iter()
        .map(|c| {
            let filtered = input
                .iter()
                .filter(|&x| x.to_ascii_lowercase() != *c)
                .map(|x| *x)
                .collect::<Vec<u8>>();
            polymer_reaction(&filtered).len()
        })
        .collect::<Vec<usize>>();
    println!("Part 2: min length {}", lengths.iter().min().unwrap());
}

fn find_units(polymer: &[u8]) -> Vec<u8> {
    let mut pairs: HashSet<u8> = HashSet::new();
    for x in polymer {
        pairs.insert(x.to_ascii_lowercase());
    }

    pairs.iter().map(|x| *x).collect::<Vec<u8>>()
}

fn polymer_reaction(polymer: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for c in polymer {
        if result.is_empty() {
            result.push(*c);
            continue;
        }
        if units_will_react(*c, *result.last().unwrap()) {
            result.pop();
        } else {
            result.push(*c);
        }
    }

    result
}

#[test]
fn test_polymer_reaction() {
    fn polymer_reaction_string(s: &str) -> String {
        polymer_reaction(&s.bytes().collect::<Vec<_>>())
            .iter()
            .map(|x| *x as char)
            .collect::<String>()
    }
    assert_eq!(polymer_reaction_string("Aa"), "".to_string());
    assert_eq!(polymer_reaction_string("aA"), "".to_string());
    assert_eq!(polymer_reaction_string("abBA"), "".to_string());
    assert_eq!(polymer_reaction_string("aabAAB"), "aabAAB".to_string());
}

#[inline]
fn units_will_react(a: u8, b: u8) -> bool {
    return (a as isize - b as isize).abs() == b'a' as isize - b'A' as isize;
}

#[test]
fn test_units_will_react() {
    assert_eq!(units_will_react(b'a', b'A'), true);
    assert_eq!(units_will_react(b'A', b'a'), true);
    assert_eq!(units_will_react(b'a', b'b'), false);
    assert_eq!(units_will_react(b'a', b'B'), false);
}

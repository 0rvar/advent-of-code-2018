use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let box_ids: Vec<&str> = input.trim().split("\n").collect::<Vec<_>>();

    let part_1_checksum: usize = box_ids.iter().map(|x| hash(x, 2)).sum::<usize>()
        * box_ids.iter().map(|x| hash(x, 3)).sum::<usize>();
    println!("Part 1: {}", part_1_checksum);

    let mut best_score = 0usize;
    let mut best_reference = "".to_string();
    let mut best_id = "".to_string();
    for reference_id in &box_ids {
        for box_id in &box_ids {
            if reference_id == box_id {
                continue;
            }
            let box_score = score(reference_id, box_id);
            if box_score > best_score {
                best_reference = reference_id.to_string();
                best_id = box_id.to_string();
                best_score = box_score;
            }
        }
    }
    let common_letters = best_reference
        .chars()
        .zip(best_id.chars())
        .filter(|(x, y)| x == y)
        .map(|(x, _)| x)
        .collect::<String>();
    println!(
        "Part 2: {} <-> {} : {}",
        best_reference, best_id, common_letters
    );
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

fn score(box_id_a: &str, box_id_b: &str) -> usize {
    box_id_a
        .chars()
        .zip(box_id_b.chars())
        .filter(|(x, y)| x == y)
        .count()
}

#[test]
fn test_score() {
    assert_eq!(score("abcde", "axcye"), 3);
}

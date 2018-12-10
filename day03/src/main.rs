use regex::Regex;
use shared::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Rect {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SquareInch {
    x: usize,
    y: usize,
}

fn main() {
    let rectangles = include_str!("input.txt")
        .trim()
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();

    let mut assigned_square_inches = HashSet::new();
    let mut overlapped_square_inches = HashSet::new();
    for rect in &rectangles {
        for x in rect.left..(rect.left + rect.width) {
            for y in rect.top..(rect.top + rect.height) {
                let inch = SquareInch { x, y };
                if assigned_square_inches.contains(&inch) {
                    overlapped_square_inches.insert(inch);
                } else {
                    assigned_square_inches.insert(inch);
                }
            }
        }
    }
    println!(
        "Part 1: {} overlapped square inches",
        overlapped_square_inches.len()
    );

    for rect in rectangles {
        let mut any_inch_overlaps = false;
        for x in rect.left..(rect.left + rect.width) {
            for y in rect.top..(rect.top + rect.height) {
                let inch = SquareInch { x, y };
                if overlapped_square_inches.contains(&inch) {
                    any_inch_overlaps = true;
                    break;
                }
            }
        }
        if !any_inch_overlaps {
            println!("Part 2: {} does not overlap anyone else", rect.id);
            break;
        }
    }
}

fn parse_line(line: &str) -> Rect {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let captures = re.captures(line).unwrap();
    let id = get_number_match(&captures, 1);
    let left = get_number_match(&captures, 2);
    let top = get_number_match(&captures, 3);
    let width = get_number_match(&captures, 4);
    let height = get_number_match(&captures, 5);
    Rect {
        id,
        left,
        top,
        width,
        height,
    }
}

#[test]
fn test_parse_line() {
    assert_eq!(
        parse_line("#11 @ 934,606: 17x25"),
        Rect {
            id: 11,
            left: 934,
            top: 606,
            width: 17,
            height: 25,
        }
    )
}

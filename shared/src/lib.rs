use regex::Captures;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub fn move_in_direction(pos: &Position, dir: &Direction) -> Position {
    match dir {
        Direction::North => Position {
            x: pos.x,
            y: pos.y - 1,
        },
        Direction::South => Position {
            x: pos.x,
            y: pos.y + 1,
        },
        Direction::East => Position {
            x: pos.x + 1,
            y: pos.y,
        },
        Direction::West => Position {
            x: pos.x - 1,
            y: pos.y,
        },
    }
}

pub fn manhattan_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> usize {
    (x1 as isize - x2 as isize).abs() as usize + (y1 as isize - y2 as isize).abs() as usize
}

#[test]
fn test_manhattan_distance() {
    assert_eq!(manhattan_distance(0, 0, 1, 1), 2);
    assert_eq!(manhattan_distance(0, 1, 1, 1), 1);
    assert_eq!(manhattan_distance(1, 1, 1, 1), 0);
    assert_eq!(manhattan_distance(1, 1, 3, 0), 3);
}

pub fn get_number_match(capture: &Captures<'_>, index: usize) -> usize {
    capture.get(index).unwrap().as_str().parse().unwrap()
}

pub fn get_string_match<'l>(capture: &Captures<'l>, index: usize) -> &'l str {
    capture.get(index).unwrap().as_str()
}

pub fn report_result_with_measurement<F>(tag: &str, func: F)
where
    F: Fn() -> String,
{
    use std::time::Instant;
    let now = Instant::now();
    let result = func();
    let new_now = Instant::now();
    println!("{}: {} ({:?})", tag, result, new_now.duration_since(now));
}

pub fn get_digits(n: usize) -> Vec<usize> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

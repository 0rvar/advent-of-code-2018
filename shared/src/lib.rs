use regex::Captures;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub x: isize,
    pub y: isize,
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

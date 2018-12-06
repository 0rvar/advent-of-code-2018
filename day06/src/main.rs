use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Claim {
    id: isize,
    distance: usize,
}

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|x| {
            let parts = x
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<isize>>();
            (parts[0], parts[1])
        })
        .collect::<Vec<_>>();

    let mut map: HashMap<Position, Claim> = HashMap::new();
    let mut region_sizes: HashMap<usize, usize> = HashMap::new();
    let mut infinite_area_ids = HashSet::new();
    let min_x = 0;
    let min_y = 0;
    let max_x = 500;
    let max_y = 500;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut claim = Claim {
                id: -1,
                distance: 100000,
            };
            for (index, coordinate) in input.iter().enumerate() {
                let distance: usize = manhattan_distance(coordinate.0, coordinate.1, x, y);
                if distance == claim.distance {
                    claim.id = -2; // The "multiple equidistant" claim
                } else if distance < claim.distance {
                    claim.id = index as isize;
                    claim.distance = distance;
                }
            }
            if x == max_x || y == max_y || x == min_x || y == min_y {
                infinite_area_ids.insert(claim.id);
            }
            if claim.id >= 0 {
                *region_sizes.entry(claim.id as usize).or_insert(0) += 1;
                map.insert(Position { x, y }, claim);
            }
        }
    }
    for x in &infinite_area_ids {
        region_sizes.remove(&(*x as usize));
    }
    println!(
        "Part 1: {}",
        region_sizes.iter().max_by_key(|&(_, v)| v).unwrap().1
    );

    let mut area_size = 0usize;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut total_distance = 0usize;
            for coordinate in &input {
                let distance: usize = manhattan_distance(coordinate.0, coordinate.1, x, y);
                total_distance += distance;
            }
            if total_distance < 10000 {
                area_size += 1;
            }
        }
    }
    println!("Part 2: {}", area_size);
}

fn manhattan_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> usize {
    (x1 as isize - x2 as isize).abs() as usize + (y1 as isize - y2 as isize).abs() as usize
}

#[test]
fn test_manhattan_distance() {
    assert_eq!(manhattan_distance(0, 0, 1, 1), 2);
    assert_eq!(manhattan_distance(0, 1, 1, 1), 1);
    assert_eq!(manhattan_distance(1, 1, 1, 1), 0);
    assert_eq!(manhattan_distance(1, 1, 3, 0), 3);
}

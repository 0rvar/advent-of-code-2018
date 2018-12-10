use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: isize,
    y: isize,
}

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|line| {
            let numbers = line
                .trim()
                .split(",")
                .map(|x| x.trim().parse().unwrap())
                .collect::<Vec<isize>>();
            Point {
                x: numbers[0],
                y: numbers[1],
                vx: numbers[2],
                vy: numbers[3],
            }
        })
        .collect::<Vec<Point>>();

    let mut points = input.clone();
    let mut seconds_elapsed = 0;
    loop {
        points = tick(&points);
        seconds_elapsed += 1;
        if stop_condition(&points) {
            break;
        }
    }
    print_points(&points);
    println!("Elapsed: {} seconds", seconds_elapsed);
}

fn print_points(points: &[Point]) {
    let (min_x, min_y, max_x, max_y) = bounding_rect(points);
    let lookup = points
        .iter()
        .map(|p| Position { x: p.x, y: p.y })
        .collect::<HashSet<_>>();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                if lookup.contains(&Position { x, y }) {
                    "#"
                } else {
                    " "
                }
            )
        }
        println!();
    }
}

fn tick(points: &[Point]) -> Vec<Point> {
    points
        .iter()
        .map(|p| Point {
            x: p.x + p.vx,
            y: p.y + p.vy,
            vx: p.vx,
            vy: p.vy,
        })
        .collect()
}

fn stop_condition(points: &[Point]) -> bool {
    let mut max_min = 0;
    for (i1, p1) in points.iter().enumerate() {
        let mut min = 999;
        for (i2, p2) in points.iter().enumerate() {
            if i1 == i2 {
                continue;
            }
            let distance = manhattan_distance(p1.x, p1.y, p2.x, p2.y);
            if distance < min {
                min = distance;
            }
        }
        if min > max_min {
            max_min = min;
            if max_min > 5 {
                break;
            }
        }
    }
    max_min < 3
}

fn manhattan_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> usize {
    (x1 as isize - x2 as isize).abs() as usize + (y1 as isize - y2 as isize).abs() as usize
}

fn bounding_rect(points: &[Point]) -> (isize, isize, isize, isize) {
    let mut min_x = 10000;
    let mut min_y = 10000;
    let mut max_x = -10000;
    let mut max_y = -10000;
    for p in points {
        if p.x < min_x {
            min_x = p.x;
        }
        if p.y < min_y {
            min_y = p.y;
        }
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }
    (min_x, min_y, max_x, max_y)
}

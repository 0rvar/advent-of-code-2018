use shared::*;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Track {
    Horizontal,
    Vertical,
    Crossing,
    PositiveTurn,
    NegativeTurn,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cart {
    position: Position,
    direction: Direction,
    turns: usize,
}

fn main() {
    let input: Vec<Vec<u8>> = include_str!("input.txt")
        .split("\n")
        .map(|x| x.as_bytes().iter().map(|x| *x).collect::<Vec<u8>>())
        .collect();

    let mut map: HashMap<Position, Track> = HashMap::new();
    let mut carts: Vec<Cart> = vec![];
    for (y, line) in input.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            if *value == b' ' || *value == b'\r' {
                continue;
            }

            let pos = Position {
                x: x as isize,
                y: y as isize,
            };
            match &value {
                b'v' => carts.push(Cart {
                    position: pos.clone(),
                    direction: Direction::South,
                    turns: 0,
                }),
                b'^' => carts.push(Cart {
                    position: pos.clone(),
                    direction: Direction::North,
                    turns: 0,
                }),
                b'<' => carts.push(Cart {
                    position: pos.clone(),
                    direction: Direction::West,
                    turns: 0,
                }),
                b'>' => carts.push(Cart {
                    position: pos.clone(),
                    direction: Direction::East,
                    turns: 0,
                }),
                _ => {}
            };

            map.insert(
                pos,
                match value {
                    b'-' => Track::Horizontal,
                    b'|' => Track::Vertical,
                    b'+' => Track::Crossing,
                    b'/' => Track::PositiveTurn,
                    b'\\' => Track::NegativeTurn,

                    b'v' => Track::Vertical,
                    b'^' => Track::Vertical,
                    b'<' => Track::Horizontal,
                    b'>' => Track::Horizontal,

                    _ => panic!(format!("invalid rail: {}", *value as char)),
                },
            );
        }
    }

    {
        let mut carts: Vec<Cart> = carts.clone();
        'it: for iteration in 1.. {
            carts.sort_by_key(|c| (c.position.x, c.position.y));
            let mut next_carts: Vec<Cart> = vec![];
            for (cart_index, cart) in carts.iter().enumerate() {
                let next_position = move_in_direction(&cart.position, &cart.direction);
                let track = map.get(&next_position).unwrap();
                let mut next_turns = cart.turns;
                let next_direction = match track {
                    Track::Horizontal | Track::Vertical => cart.direction.clone(),
                    Track::Crossing => {
                        next_turns += 1;
                        choose_direction(&cart.direction, cart.turns)
                    }
                    Track::PositiveTurn => match cart.direction {
                        // /
                        Direction::North => Direction::East,
                        Direction::East => Direction::North,
                        Direction::South => Direction::West,
                        Direction::West => Direction::South,
                    },
                    Track::NegativeTurn => match cart.direction {
                        // \
                        Direction::North => Direction::West,
                        Direction::West => Direction::North,
                        Direction::South => Direction::East,
                        Direction::East => Direction::South,
                    },
                };

                let collision_in_next = next_carts[0..cart_index]
                    .iter()
                    .find(|c| c.position == next_position)
                    .is_some();
                let collision_in_prev = carts[(cart_index + 1)..]
                    .iter()
                    .find(|c| c.position == next_position)
                    .is_some();
                if collision_in_next || collision_in_prev {
                    println!("Part 1: {},{}", next_position.x, next_position.y);
                    break 'it;
                }

                next_carts.push(Cart {
                    position: next_position,
                    direction: next_direction,
                    turns: next_turns,
                });
            }

            for y in 0..input.len() {
                for x in 0..input[0].len() {
                    let pos = Position {
                        x: x as isize,
                        y: y as isize,
                    };
                    if let Some(c) = next_carts.iter().find(|c| c.position == pos) {
                        print!(
                            "\x1b[1m\x1b[92m{}\x1b[0m",
                            match c.direction {
                                Direction::North => "^",
                                Direction::South => "v",
                                Direction::East => ">",
                                Direction::West => "<",
                            }
                        );
                    } else if let Some(t) = map.get(&pos) {
                        print!(
                            "{}",
                            match t {
                                Track::Horizontal => "-",
                                Track::Vertical => "|",
                                Track::Crossing => "+",
                                Track::PositiveTurn => "/",
                                Track::NegativeTurn => "\\",
                            }
                        );
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            println!("========================");
            println!();

            carts = next_carts;

            // use std::io;
            // io::stdin().read_line(&mut String::new());
        }
    }
}

fn choose_direction(dir: &Direction, counter: usize) -> Direction {
    enum Turn {
        Left,
        Right,
        Forward,
    }
    let turn = &[Turn::Left, Turn::Forward, Turn::Right][counter % 3];
    match turn {
        Turn::Forward => dir.clone(),
        Turn::Left => match dir {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        },
        Turn::Right => match dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        },
    }
}

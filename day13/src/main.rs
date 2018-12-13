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
    let input: Vec<Vec<u8>> = include_str!("input2.txt")
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
        let mut carts = carts.clone();
        'it: for iteration in 1.. {
            let next_carts = carts
                .iter()
                .map(|cart| {
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

                    Cart {
                        position: next_position,
                        direction: next_direction,
                        turns: next_turns,
                    }
                })
                .collect::<Vec<Cart>>();

            let mut cart_positions: HashSet<Position> = HashSet::new();
            for cart in carts {
                if cart_positions.contains(&cart.position) {
                    println!("Part 1: {:?}", cart.position);
                    break 'it;
                }
                cart_positions.insert(cart.position.clone());
            }

            carts = next_carts;
        }
    }
}

fn choose_direction(dir: &Direction, counter: usize) -> Direction {
    enum Turn {
        Left,
        Right,
        Forward,
    }
    let turn = &[Turn::Left, Turn::Right, Turn::Forward][counter % 3];
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

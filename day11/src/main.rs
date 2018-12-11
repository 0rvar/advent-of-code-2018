use shared::*;

fn main() {
    const SERIAL_NUMBER: usize = 8868;

    {
        let mut max_value = 0;
        let mut max_coord = (0, 0);
        for x in 1..=(300 - 2) {
            for y in 1..=(300 - 2) {
                let mut value = 0;
                for offset_x in 0..=2 {
                    for offset_y in 0..=2 {
                        value += calculate_power_level(x + offset_x, y + offset_y, SERIAL_NUMBER);
                    }
                }
                if value > max_value {
                    max_value = value;
                    max_coord = (x, y);
                }
            }
        }
        println!("Part 1: max {} at {:?}", max_value, max_coord);
    }
}

fn calculate_power_level(x: usize, y: usize, serial_number: usize) -> isize {
    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + serial_number;
    let power_level = power_level * rack_id;

    get_hundred_digit(power_level) as isize - 5
}

#[test]
fn test_calculate_power_level() {
    assert_eq!(calculate_power_level(3, 5, 8), 4);
    assert_eq!(calculate_power_level(122, 79, 57), -5);
    assert_eq!(calculate_power_level(217, 196, 39), 0);
    assert_eq!(calculate_power_level(101, 153, 71), 4);
}

fn get_hundred_digit(number: usize) -> usize {
    let digits = number.to_string().chars().collect::<Vec<char>>();
    if digits.len() > 2 {
        digits
            .get(digits.len() - 3)
            .unwrap_or(&'0')
            .to_string()
            .parse::<usize>()
            .unwrap()
    } else {
        0
    }
}

#[test]
fn test_get_hundred_digit() {
    assert_eq!(get_hundred_digit(5), 0);
    assert_eq!(get_hundred_digit(500), 5);
    assert_eq!(get_hundred_digit(1333), 3);
}

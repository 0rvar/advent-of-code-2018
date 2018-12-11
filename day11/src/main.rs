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

    {
        let mut power_level_cache = [0isize; 300 * 300];
        for x in 1..=300 {
            for y in 1..=300 {
                power_level_cache[(y - 1) * 300 + (x - 1)] =
                    calculate_power_level(x, y, SERIAL_NUMBER);
            }
        }

        let mut max_value = 0;
        let mut best_size = 0;
        let mut max_coord = (0, 0);
        for size in 1..=299 {
            for x in 1..=(300 - size) {
                for y in 1..=(300 - size) {
                    let mut value = 0;
                    for offset_y in 0..size {
                        let slice_start = (y - 1 + offset_y) * 300 + (x - 1);
                        let slice_end = (y - 1 + offset_y) * 300 + (x - 1 + size);
                        value += power_level_cache[slice_start..slice_end]
                            .iter()
                            .sum::<isize>();
                    }
                    if value > max_value {
                        max_value = value;
                        max_coord = (x, y);
                        best_size = size;
                    }
                }
            }
        }
        println!(
            "Part 2: max {} at {},{},{}",
            max_value, max_coord.0, max_coord.1, best_size
        );
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
    number / 100 % 10
}

#[test]
fn test_get_hundred_digit() {
    assert_eq!(get_hundred_digit(5), 0);
    assert_eq!(get_hundred_digit(500), 5);
    assert_eq!(get_hundred_digit(1333), 3);
}

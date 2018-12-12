use shared::*;

fn transform(current: &str) -> char {
    match current {
        "..#.#" => '.',
        "#..##" => '#',
        "..###" => '.',
        "###.#" => '.',
        "#...#" => '#',
        "###.." => '#',
        ".##.#" => '#',
        "#..#." => '#',
        "#.##." => '#',
        "####." => '.',
        ".#.##" => '#',
        "...#." => '.',
        ".#..#" => '#',
        ".###." => '.',
        "##..#" => '#',
        ".##.." => '#',
        ".####" => '#',
        ".#.#." => '#',
        "#####" => '.',
        "#.#.#" => '#',
        "...##" => '#',
        "..##." => '.',
        "....#" => '.',
        "##..." => '.',
        "##.#." => '#',
        "..#.." => '#',
        "....." => '.',
        "##.##" => '.',
        "#.###" => '.',
        "#.#.." => '.',
        ".#..." => '#',
        "#...." => '.',
        _ => panic!("Unknown combination"),
    }
}

fn main() {
    let input = "###......#.#........##.###.####......#..#####.####..#.###..#.###.#..#..#.#..#..#.##...#..##......#.#";

    let mut current_state = input.to_string();
    let mut start_index: isize = 0;
    for _iteration in 1..=20 {
        let mut next_state: Vec<char> = vec![];
        for i in 0..(current_state.len() + 4) {
            let cells = get_cells(&current_state, i - 2);
            next_state.push(transform(&cells));
        }
        current_state = next_state.iter().collect::<String>();
        start_index -= 2;
        println!("{:?}", current_state);
    }
    println!(
        "Part 1: {} alive",
        current_state
            .chars()
            .enumerate()
            .filter(|(_, x)| *x == '#')
            .map(|(i, _)| i as isize + start_index)
            .sum::<isize>()
    );
}

fn get_cells(s: &str, index: usize) -> String {
    let start: isize = index as isize - 2;
    let end = (index + 2).min(s.len() - 1);
    let mut cells = s.as_bytes()[(start.max(0) as usize)..=end].to_vec();
    for _ in start..0 {
        cells.insert(0, b'.');
    }
    for _ in cells.len()..5 {
        cells.push(b'.');
    }
    cells.iter().map(|x| *x as char).collect::<String>()
}

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

fn is_static(current: &str) -> bool {
    match current {
        "....#" => true,
        "...#." => true,
        "..#.#" => true,
        ".#.#." => true,
        "#.#.." => true,
        ".#..." => true,
        "#...." => true,
        "....." => true,
        _ => false,
    }
}

fn main() {
    let input = "###......#.#........##.###.####......#..#####.####..#.###..#.###.#..#..#.#..#..#.##...#..##......#.#";

    {
        let mut current_state = input.to_string();
        let mut start_index: isize = 0;
        for _iteration in 1..=20 {
            let mut next_state: Vec<char> = vec![];
            for i in 0..(current_state.len() + 2) {
                let cells = get_cells(&current_state, i - 1);
                next_state.push(transform(&cells));
            }
            current_state = next_state.iter().collect::<String>();
            start_index -= 1;
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

    {
        let mut current_state = input.to_string();
        let mut start_index: isize = 0;
        for iteration in 1..=1000 {
            let mut next_state: Vec<char> = vec![];
            let mut all_static = true;
            for i in 0..(current_state.len() + 2) {
                let cells = get_cells(&current_state, i - 1);
                next_state.push(transform(&cells));
                all_static = all_static && is_static(&cells);
            }
            current_state = next_state.iter().collect::<String>();
            start_index -= 1;

            if all_static {
                let remaining_generations = 50000000000usize - iteration;

                println!(
                    "Part 2: {} alive",
                    current_state
                        .chars()
                        .enumerate()
                        .filter(|(_, x)| *x == '#')
                        .map(|(i, _)| (i as isize + start_index) as usize + remaining_generations)
                        .sum::<usize>()
                );
                break;
            }
        }
    }
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

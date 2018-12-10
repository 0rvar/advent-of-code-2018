use std::collections::VecDeque;

// input: 426 players; last marble is worth 72058 points
const NUM_PLAYERS: usize = 426;
const MAX_MARBLE: usize = 72058;

fn main() {
    println!("Part 1: {}", max_score(MAX_MARBLE));
    println!("Part 2: {}", max_score(MAX_MARBLE * 100));
}

fn max_score(max_marble: usize) -> usize {
    let mut scores = [0usize; NUM_PLAYERS];
    let mut board: VecDeque<usize> = VecDeque::new();
    board.insert(0, 0);

    let mut current_player = 0;
    for current_marble in 1..=max_marble {
        if current_marble % 23 == 0 {
            // Move 7 positions to the left
            for _ in 0..7 {
                let last = board.pop_back().unwrap();
                board.push_front(last);
            }
            let removed_marble = board.pop_front().unwrap();
            scores[current_player] += current_marble;
            scores[current_player] += removed_marble;
        } else {
            for _ in 0..2 {
                let first = board.pop_front().unwrap();
                board.push_back(first);
            }
            board.push_front(current_marble);
        }
        current_player = (current_player + 1) % NUM_PLAYERS;
    }
    *scores.iter().max().unwrap()
}

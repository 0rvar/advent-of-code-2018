use skiplist::SkipList;

// input: 426 players; last marble is worth 72058 points
const NUM_PLAYERS: usize = 426;
const MAX_MARBLE: usize = 72058;

fn main() {
    println!("Part 1: {}", max_score(MAX_MARBLE));
    println!("Part 2: {}", max_score(MAX_MARBLE * 100));
}

fn max_score(max_marble: usize) -> usize {
    let mut scores = [0usize; NUM_PLAYERS];
    let mut board: SkipList<usize> = SkipList::new();
    board.insert(0, 0);

    let mut current_player = 0;
    let mut current_position = 0;
    for current_marble in 1..=max_marble {
        if current_marble % 23 == 0 {
            let take_position = (current_position + board.len() - 7) % board.len();
            let removed_marble = board.remove(take_position);

            scores[current_player] += current_marble;
            scores[current_player] += removed_marble;
            current_position = take_position;
        } else {
            let next_position = (current_position + 2) % board.len();
            board.insert(current_marble, next_position);
            current_position = next_position;
        }
        current_player = (current_player + 1) % NUM_PLAYERS;
    }
    *scores.iter().max().unwrap()
}

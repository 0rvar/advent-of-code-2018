fn main() {
    // input: 426 players; last marble is worth 72058 points
    const NUM_PLAYERS: usize = 426;
    const MAX_MARBLE: usize = 72058;
    // const NUM_PLAYERS: usize = 10;
    // const MAX_MARBLE: usize = 1618;

    let mut scores = [0usize; NUM_PLAYERS];
    let mut board: Vec<usize> = vec![0];
    let mut current_player = 0;
    let mut current_position = 0;
    for current_marble in 1..=MAX_MARBLE {
        if current_marble % 23 == 0 {
            let take_position = {
                let x = ((current_position + board.len()) - 7) % board.len();
                x
            };
            let removed_marble = board.remove(take_position);

            scores[current_player] += current_marble;
            scores[current_player] += removed_marble;
            current_position = take_position;
        } else {
            let next_position = {
                let mut x = if board.len() < 2 {
                    1
                } else {
                    (current_position + 2) % board.len()
                };
                if x == 0 {
                    x = current_position + 2
                }
                x
            };
            board.insert(next_position, current_marble);
            current_position = next_position;
        }
        // println!("{}: {:?} - {:?}", current_player, board, scores);
        current_player = (current_player + 1) % NUM_PLAYERS;
    }
    println!("Part 1: {}", scores.iter().max().unwrap());
}

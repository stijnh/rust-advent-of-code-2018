fn play_game(num_players: usize, num_marbles: usize) -> usize {
    let mut scores = vec![0; num_players];
    let mut next = vec![!0; num_marbles];
    let mut prev = vec![!0; num_marbles];
    let mut cursor = 0;

    prev[0] = 0;
    next[0] = 0;

    for i in 1..num_marbles {
        if i % 23 != 0 {
            let a = next[cursor];
            let b = next[a];

            next[a] = i;
            prev[i] = a;

            prev[b] = i;
            next[i] = b;

            cursor = i;
        } else {
            for _ in 0..7 {
                cursor = prev[cursor];
            }

            let player = i % num_players;
            scores[player] += i;
            scores[player] += cursor;

            let (a, b) = (prev[cursor], next[cursor]);
            next[a] = b;
            prev[b] = a;

            cursor = b;

            if scores[player] == 8317 {
                println!("{}", "!!!");
            }
        }
    }

    *scores.iter().max().unwrap()
}

pub fn run(_: &[&str]) {
    println!("answer A: {}", play_game(455, 71223));
    println!("answer B: {}", play_game(455, 7122300));
}

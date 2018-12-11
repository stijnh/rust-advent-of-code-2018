use std::collections::VecDeque;

fn rotate<T>(vec: &mut VecDeque<T>, n: i32) {
    if !vec.is_empty() {
        if n > 0 {
            for _ in 0..n {
                let m = vec.pop_front().unwrap();
                vec.push_back(m);
            }
        } else {
            for _ in 0..-n {
                let m = vec.pop_back().unwrap();
                vec.push_front(m);
            }
        }
    }
}

fn play_game(num_players: usize, num_marbles: usize) -> usize {
    let mut scores = vec![0; num_players];
    let mut marbles = VecDeque::new();

    marbles.push_back(0);

    for i in 1..num_marbles {
        if i % 23 != 0 {
            rotate(&mut marbles, 2);
            marbles.push_front(i);
        } else {
            rotate(&mut marbles, -7);

            let player = i % num_players;
            scores[player] += i;
            scores[player] += marbles.pop_front().unwrap();
        }
    }

    *scores.iter().max().unwrap()
}

pub fn run(_: &[&str]) {
    println!("answer A: {}", play_game(455, 71223));
    println!("answer B: {}", play_game(455, 7122300));
}

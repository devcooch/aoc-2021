const P: [u64; 11] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1, 0];

fn turn(
    active_score: usize,
    inactive_score: usize,
    active_pos: usize,
    inactive_pos: usize,
    universes: u64,
    active_wins: &mut u64,
    inactive_wins: &mut u64,
) {
    if inactive_score >= 21 {
        *inactive_wins += universes;
        return;
    }
    for dice in 3..=9 {
        turn(
            inactive_score,
            active_score + (active_pos + dice) % 10 + 1,
            inactive_pos,
            (active_pos + dice) % 10,
            universes * P[dice],
            inactive_wins,
            active_wins,
        );
    }
}

fn main() {
    let mut wins_1 = 0u64;
    let mut wins_2 = 0u64;
    turn(0, 0, 5, 0, 1, &mut wins_1, &mut wins_2);
    println!("{}", wins_1);
    println!("{}", wins_2);
}

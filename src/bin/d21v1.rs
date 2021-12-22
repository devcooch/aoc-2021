fn main() {
    let mut who = 0;
    let mut p = [5, 0];
    let mut score = [0, 0];
    let mut dice = 99;
    let mut rolls = 0;
    while score.iter().all(|s| *s < 1000) {
        for _ in 0..3 {
            rolls += 1;
            dice += 1;
            dice %= 100;
            p[who] += dice + 1;
            p[who] %= 10;
        }
        score[who] += p[who] + 1;
        who = 1 - who;
    }
    println!("{}", score.iter().min().unwrap() * rolls);
}

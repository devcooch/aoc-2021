fn xt(vx0: i32, step: i32) -> i32 {
    (1..=step)
        .map(|t| vx0 - vx0.signum() * (t - 1))
        .take_while(|v| v.signum() == vx0.signum())
        .sum()
}

fn yt(vy0: i32, step: i32) -> i32 {
    (1..=step).map(|t| vy0 - (t - 1)).sum()
}

fn main() {
    let test_x = [7, 13, 18, 22, 25, 27, 28, 28, 28];
    test_x
        .iter()
        .enumerate()
        .for_each(|(t, x)| assert!(xt(7, t as i32 + 1) == *x));
    let test_y = [2, 3, 3, 2, 0, -3, -7];
    test_y
        .iter()
        .enumerate()
        .for_each(|(t, y)| assert!(yt(2, t as i32 + 1) == *y));
    let (from_x, to_x, from_y, to_y) = (240, 292, -90, -57);
    let mut h = Vec::new();
    for vx in -300..300 {
        for vy in -300..300 {
            let p: Vec<_> = (1..=200).map(|t| (xt(vx, t), yt(vy, t))).collect();
            if p.iter()
                .any(|(x, y)| (x >= &from_x) && (x <= &to_x) && (y >= &from_y) && (y <= &to_y))
            {
                h.push(p.iter().map(|(_, y)| *y).max().unwrap());
            }
        }
    }
    println!("{}", h.iter().max().unwrap());
}

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
    let (from_x, to_x, from_y, to_y) = (240, 292, -90, -57);
    let mut ps = Vec::new();
    for vx in -300..300 {
        for vy in -300..300 {
            let p: Vec<_> = (1..=200).map(|t| (xt(vx, t), yt(vy, t))).collect();
            if p.iter()
                .any(|(x, y)| (x >= &from_x) && (x <= &to_x) && (y >= &from_y) && (y <= &to_y))
            {
                ps.push((vx, vy));
            }
        }
    }
    println!("{}", ps.len());
}

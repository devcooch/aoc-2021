const OPS: [fn(usize) -> usize; 3] = [|x: usize| x.wrapping_sub(1), |x| x, |x| x.wrapping_add(1)];

fn main() {
    let data = include_str!("day20.txt");
    let algo = data
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| (c == '#') as usize)
        .collect::<Vec<_>>();
    let m = data
        .lines()
        .skip(2)
        .map(|line| {
            line.chars()
                .map(|c| (c == '#') as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let n0 = m.len();
    let mut last = m;
    for step in 0..50 {
        let n = n0 + 2 * (step + 1);
        let mut new = vec![vec![0usize; n]; n];
        for r in 0..n {
            for c in 0..n {
                let mut res = 0usize;
                for ox in OPS {
                    for oy in OPS {
                        res <<= 1;
                        let x = ox(r.wrapping_sub(1));
                        let y = oy(c.wrapping_sub(1));
                        if let Some(bit) = last.get(x).and_then(|r| r.get(y)) {
                            res |= bit;
                        } else {
                            res |= algo[0] * (step % 2);
                        }
                    }
                }
                new[r][c] = algo[res];
            }
        }
        last = new;
    }
    let total: usize = last.iter().map(|row| row.iter().sum::<usize>()).sum();
    println!("{}", total);
}

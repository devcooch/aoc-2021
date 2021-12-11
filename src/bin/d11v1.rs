const MODIFIERS: [fn(usize) -> usize; 3] = [|x: usize| x.wrapping_sub(1), |x| x, |x| x + 1];

fn main() {
    let data = include_str!("day11.txt");
    let mut octos: Vec<Vec<usize>> = data
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    for _ in 1..=100 {
        let mut flashing = Vec::new();
        octos.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, o)| {
                *o += 1;
                if *o % 10 == 0 {
                    flashing.push((i, j))
                }
            })
        });
        while let Some((i, j)) = flashing.pop() {
            for fx in MODIFIERS {
                for fy in MODIFIERS {
                    let x = fx(i);
                    let y = fy(j);
                    if let Some(o) = octos.get_mut(x).and_then(|r| r.get_mut(y)) {
                        if (*o % 10) != 0 {
                            *o += 1;
                            if *o % 10 == 0 {
                                flashing.push((x, y));
                            }
                        }
                    }
                }
            }
        }
    }
    let result: usize = octos
        .iter()
        .map(|r| r.iter().map(|&o| o / 10).sum::<usize>())
        .sum();
    println!("{}", result);
}

fn main() {
    let n = 100;
    let mut m: Vec<Vec<usize>> = vec![vec![10usize; n + 2]];
    let data = include_str!("day09.txt");
    data.lines().for_each(|line| {
        let mut v = vec![10usize];
        line.chars().for_each(|c| v.push(c as usize - '0' as usize));
        v.push(10);
        m.push(v);
    });
    m.push(vec![10usize; n + 2]);
    assert!(m.len() == n + 2);
    assert!(m[0].len() == n + 2);
    let s: usize = m
        .iter()
        .enumerate()
        .skip(1)
        .take(n)
        .map(|(r, v)| {
            v.iter()
                .enumerate()
                .skip(1)
                .take(n)
                .filter(|(c, x)| {
                    (x < &&m[r - 1][*c])
                        && (x < &&m[r + 1][*c])
                        && (x < &&m[r][c - 1])
                        && (x < &&m[r][c + 1])
                })
                .map(|(_, x)| x + 1)
                .sum::<usize>()
        })
        .sum();
    println!("{}", s);
}

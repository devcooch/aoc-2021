use std::collections::HashSet;

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
    let mut basins: Vec<_> = Vec::new();
    m.iter().enumerate().skip(1).take(n).for_each(|(r, v)| {
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
            .map(|(c, _)| (r, c))
            .for_each(|x| basins.push(x))
    });
    let dxys = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut sizes = basins
        .iter()
        .map(|(r, c)| {
            let mut visited: HashSet<_> = HashSet::from([]);
            let mut candidates: Vec<_> = vec![(*r, *c)];
            while let Some((r, c)) = candidates.pop() {
                visited.insert((r, c));
                for (dx, dy) in dxys {
                    let nr = (r as isize + dx) as usize;
                    let nc = (c as isize + dy) as usize;
                    let new = (nr, nc);
                    if m[new.0][new.1] < 9 && !visited.contains(&new) {
                        candidates.push(new);
                    }
                }
            }
            visited.len()
        })
        .collect::<Vec<_>>();
    sizes.sort_unstable();
    println!("{}", sizes.iter().rev().take(3).product::<usize>());
}

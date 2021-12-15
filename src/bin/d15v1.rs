use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let data = include_str!("day15.txt");
    let score = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<_>>>();
    assert!(score[0].len() == score.len());
    let n = score.len();
    let mut vis = vec![vec![0u32; n]; n];
    let mut risk = vec![vec![0u32; n]; n];
    vis[0][0] = 1;
    let mut to_test = BinaryHeap::new();
    to_test.push(Reverse((0, (1, 0))));
    to_test.push(Reverse((0, (0, 1))));
    while vis[n - 1][n - 1] == 0 {
        let Reverse((total, (x, y))) = to_test.pop().unwrap();
        if vis[x][y] == 1 {
            continue;
        }
        vis[x][y] = 1;
        risk[x][y] = total + score[x][y];
        if (x + 1 < n) && (vis[x + 1][y] == 0) {
            to_test.push(Reverse((risk[x][y], (x + 1, y))));
        }
        if (y + 1 < n) && (vis[x][y + 1] == 0) {
            to_test.push(Reverse((risk[x][y], (x, y + 1))));
        }
        if (x > 0) && (vis[x - 1][y] == 0) {
            to_test.push(Reverse((risk[x][y], (x - 1, y))));
        }
        if (y > 0) && (vis[x][y - 1] == 0) {
            to_test.push(Reverse((risk[x][y], (x, y - 1))));
        }
    }
    println!("{}", risk[n - 1][n - 1]);
}

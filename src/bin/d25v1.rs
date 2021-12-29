fn main() {
    let data = include_str!("day25.txt");
    let mut m = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rows = m.len();
    let cols = m[0].len();
    println!("{}x{}", rows, cols);
    let mut step = 0;
    loop {
        step += 1;
        let mut move_right = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                if m[i][j] == '>' {
                    let new_j = (j + 1) % cols;
                    if m[i][new_j] == '.' {
                        move_right.push((i, j));
                    }
                }
            }
        }
        for &(i, j) in move_right.iter() {
            m[i][(j + 1) % cols] = '>';
            m[i][j] = '.';
        }
        let mut move_down = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                if m[i][j] == 'v' {
                    let new_i = (i + 1) % rows;
                    if m[new_i][j] == '.' {
                        move_down.push((i, j));
                    }
                }
            }
        }
        for &(i, j) in move_down.iter() {
            m[(i + 1) % rows][j] = 'v';
            m[i][j] = '.';
        }
        if move_right.len() == 0 && move_down.len() == 0 {
            break;
        }
    }
    println!("{}", step);
}

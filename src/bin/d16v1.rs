fn main() {
    let hex = include_str!("day16.txt");
    let line = hex.lines().next().unwrap();
    let bin = line
        .chars()
        .map(|x| match x {
            '0'..='9' | 'A'..='F' => x.to_digit(16).unwrap(),
            _ => panic!("Unknown char {}", x),
        })
        .map(|n| [(n & 0x8) >> 3, (n & 0x4) >> 2, (n & 0x2) >> 1, n & 0x1])
        .flatten()
        .collect::<Vec<_>>();
}

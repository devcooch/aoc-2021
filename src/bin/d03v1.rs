fn main() {
    let mut sum = vec![0; 12];
    let bytes = include_str!("day03.txt");
    let mut line_count = 0usize;
    for line in bytes.lines() {
        for (i, c) in line.chars().enumerate() {
            sum[i] += (c == '1') as usize;
        }
        line_count += 1;
    }
    let mut gamma = 0usize;
    for x in sum {
        if x > (line_count / 2) {
            gamma += 1;
        }
        gamma <<= 1;
    }
    gamma >>= 1;
    let epsilon = !gamma & 0x0FFF;
    println!("{}", gamma * epsilon);
}

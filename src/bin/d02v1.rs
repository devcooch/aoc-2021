fn main() {
    let data = include_str!("day02.txt");
    let mut depth = 0;
    let mut position = 0;
    for line in data.lines() {
        let mut iter = line.split_ascii_whitespace();
        let command = iter.next().unwrap();
        let value = iter.next().unwrap().parse::<i64>().unwrap();
        match command {
            "forward" => position += value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => panic!("Unknown command {}", command),
        }
    }
    println!("{}", depth * position);
}

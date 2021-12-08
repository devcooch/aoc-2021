fn main() {
    let data = include_str!("day08.txt");
    let count: usize = data
        .lines()
        .map(|line| {
            line.split(" | ")
                .skip(1)
                .next()
                .unwrap()
                .split(" ")
                .filter(|&x| x.len() < 5 || x.len() > 6)
                .count()
        })
        .sum();
    println!("{}", count);
}

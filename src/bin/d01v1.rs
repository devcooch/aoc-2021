fn main() {
    let contents = include_str!("day01.txt");
    let nums: Vec<i64> = contents
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    println!(
        "{}",
        nums.iter()
            .zip(nums.iter().skip(1))
            .map(|(n, m)| ((m - n) > 0) as usize)
            .sum::<usize>()
    );
}

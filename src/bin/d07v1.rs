fn main() {
    let data = include_str!("day07.txt");
    let mut crabs = data
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    crabs.sort_unstable();
    assert!(crabs.len() % 2 == 0);
    let ix = crabs.len() / 2;
    let pos = (crabs[ix] + crabs[ix - 1]) / 2;
    let fuel: isize = crabs.iter().map(|c| (pos - c).abs()).sum();
    println!("{}", fuel);
}

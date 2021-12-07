fn median(v: &Vec<isize>) -> isize {
    let ix = v.len() / 2;
    println!("{} {}", ix, v.len());
    if v.len() % 2 == 0 {
        (v[ix] + v[ix - 1]) / 2
    } else {
        v[ix + 1]
    }
}
fn main() {
    let data = include_str!("day07.txt");
    let mut crabs = data
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    crabs.sort();
    println!("{}", median(&crabs));
}

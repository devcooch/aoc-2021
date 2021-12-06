fn main() {
    let data = include_str!("day06.txt");
    let days = 256;
    let mut school = vec![0; 9];
    data.lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .for_each(|x| school[x] += 1);
    for _ in 0..days {
        let zero = school[0];
        for t in 0..8 {
            school[t] = school[t + 1];
        }
        school[8] = zero;
        school[6] += zero;
    }
    println!("{}", school.iter().sum::<usize>());
}

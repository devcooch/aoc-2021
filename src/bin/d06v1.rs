fn main() {
    let data = include_str!("day06.txt");
    let days = 80;
    let mut school = data
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for _ in 0..days {
        let initial_count = school.len();
        for i in 0..initial_count {
            if school[i] == 0 {
                school.push(8);
                school[i] = 6;
            } else {
                school[i] -= 1;
            }
        }
    }
    println!("{}", school.len());
}

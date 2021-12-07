use itertools::Itertools;

fn get_fuel(crabs: &[isize], pos: isize) -> isize {
    crabs
        .iter()
        .map(|c| (pos - c).abs())
        .map(|t| t * (t + 1) / 2)
        .sum()
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
    crabs.sort_unstable();
    let mut l = 0;
    let mut r = crabs.len() - 1;
    let mut pos = crabs.iter().sum::<isize>() as usize / crabs.len();
    loop {
        print!("Position {}", pos);
        let (fuel_prev, fuel_pos, fuel_next) = (pos - 1..pos + 2)
            .map(|x| get_fuel(&crabs, x as isize))
            .collect_tuple()
            .unwrap();
        println!(" -> {}", get_fuel(&crabs, pos as isize));
        if (fuel_pos < fuel_next) && (fuel_pos < fuel_prev) {
            println!("Epic WIN: {}", fuel_pos);
            break;
        }
        if fuel_prev < fuel_pos {
            r = pos;
        } else {
            l = pos;
        }
        pos = (l + r) / 2;
    }
}

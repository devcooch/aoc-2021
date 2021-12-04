use std::collections::HashSet;

fn find_candidate<P>(digits: &[Vec<usize>], criteria: P) -> usize
where
    P: Fn(usize, usize, usize) -> bool,
{
    let mut candidates = (0..digits[0].len()).collect::<HashSet<usize>>();
    for column in digits {
        let half = (candidates.len() as f32 / 2.0).ceil() as usize;
        let mut to_print = candidates.iter().collect::<Vec<&usize>>();
        to_print.sort();
        let s: usize = candidates.iter().map(|&i| column[i]).sum();
        let losers = column
            .iter()
            .enumerate()
            .filter(|(_, &x)| criteria(x, s, half))
            .map(|(i, _)| i)
            .collect::<HashSet<usize>>();
        candidates = &candidates - &losers;
        if candidates.len() == 1 {
            break;
        }
    }
    assert!(candidates.len() == 1);
    *candidates.iter().next().unwrap()
}

fn get_byte(digits: &[Vec<usize>], i: usize) -> usize {
    let mut result = 0;
    for column in digits {
        result += column[i];
        result <<= 1;
    }
    result >>= 1;
    result
}

fn main() {
    let bytes = include_str!("day03.txt");
    let mut digits = vec![Vec::<usize>::new(); 12];
    for line in bytes.lines() {
        for (i, c) in line.chars().enumerate() {
            digits[i].push((c == '1') as usize);
        }
    }
    let oxygen = find_candidate(&digits, |x, s, h| x != (s >= h) as usize);
    let co2 = find_candidate(&digits, |x, s, h| x != (s < h) as usize);
    println!("{} {}", oxygen, co2);
    println!("{}", get_byte(&digits, oxygen) * get_byte(&digits, co2));
}

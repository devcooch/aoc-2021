use itertools::Itertools;
use std::collections::HashMap;

type TwoLetter = (char, char);
type Pair = (usize, usize);

fn get_least_most_letters_count(
    counts: &[usize],
    names: &HashMap<usize, TwoLetter>,
    first_char: char,
    last_char: char,
) -> (usize, usize) {
    let mut letter_counts: HashMap<char, usize> = HashMap::new();
    counts.iter().enumerate().for_each(|(i, x)| {
        let (c1, c2) = names.get(&i).unwrap();
        let v1 = letter_counts.entry(*c1).or_insert(0);
        *v1 += x;
        let v2 = letter_counts.entry(*c2).or_insert(0);
        *v2 += x;
    });
    letter_counts.entry(first_char).and_modify(|e| *e += 1);
    letter_counts.entry(last_char).and_modify(|e| *e += 1);
    for v in letter_counts.values_mut() {
        *v /= 2;
    }
    let least = letter_counts.values().min().unwrap();
    let most = letter_counts.values().max().unwrap();
    (*least, *most)
}

fn main() {
    let mut lines = include_str!("day14.txt").lines();
    let init = lines.next().unwrap();
    lines.next();
    let mut pairs: HashMap<TwoLetter, usize> = HashMap::new();
    let mut rules: HashMap<usize, Pair> = HashMap::new();
    for line in lines {
        let mut iter = line.split(" -> ");
        let from_chars: TwoLetter = iter.next().unwrap().chars().collect_tuple().unwrap();
        let to_char: char = iter.next().unwrap().chars().next().unwrap();
        let mut n = pairs.len();
        let from_n: usize = *pairs.entry(from_chars).or_insert(n);
        n = pairs.len();
        let to_n1: usize = *pairs.entry((from_chars.0, to_char)).or_insert(n);
        n = pairs.len();
        let to_n2: usize = *pairs.entry((to_char, from_chars.1)).or_insert(n);
        rules.insert(from_n, (to_n1, to_n2));
    }
    let names: HashMap<usize, TwoLetter> = pairs.iter().map(|(&k, &v)| (v, k)).collect();
    let first_char = init.chars().next().unwrap();
    let last_char = init.chars().rev().next().unwrap();
    let mut counts = vec![0usize; pairs.len()];
    init.chars()
        .zip(init.chars().skip(1))
        .collect::<Vec<TwoLetter>>()
        .iter()
        .map(|&(t1, t2)| pairs.get(&(t1, t2)).unwrap())
        .for_each(|&p| counts[p] += 1);
    const STEPS: usize = 40;
    for step in 1..=STEPS {
        let mut new_counts = vec![0usize; pairs.len()];
        counts.iter().enumerate().for_each(|(i, x)| {
            let (left, right) = rules.get(&i).unwrap();
            new_counts[*left] += x;
            new_counts[*right] += x;
        });
        counts = new_counts;
        if step % 10 == 0 {
            let (least, most) =
                get_least_most_letters_count(&counts, &names, first_char, last_char);
            println!("Step {}: {}", step, most - least);
        }
    }
}

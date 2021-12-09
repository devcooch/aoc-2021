use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    /*
     * Segment numbering
     *
     *  0000
     * 1    2
     * 1    2
     *  3333
     * 4    5
     * 4    5
     *  6666
     *
     * set_x == set with size x
     *
     * multi-item sets:
     *  set_5:
     *  0000
     * .    .
     * .    .
     *  3333
     * .    .
     * .    .
     *  6666
     *
     *  set_6:
     *  0000
     * 1    .
     * 1    .
     *  ....
     * .    5
     * .    5
     *  6666
     *
     * Deduction for segment:
     * 0 = set_3 - set_2
     * 6 = (set_5 & set_6) - 0
     * 3 = set_5 - 0 - 6
     * 5 = set_6 & set_2
     * 2 = set_3 - 0 - 5
     * 1 = set_6 - 0 - 5 - 6
     * 4 = set_7 - set_6 - 2 - 3
     */
    let digits = HashMap::from([
        ("012456", 0),
        ("25", 1),
        ("02346", 2),
        ("02356", 3),
        ("1235", 4),
        ("01356", 5),
        ("013456", 6),
        ("025", 7),
        ("0123456", 8),
        ("012356", 9),
    ]);
    let data = include_str!("day08.txt");
    let mut s = 0;
    for line in data.lines() {
        let (encoding, number) = line.split(" | ").collect_tuple().unwrap();
        let mut dict: HashMap<usize, HashSet<char>> = HashMap::new();
        for digit in encoding.split_ascii_whitespace() {
            let n = digit.len();
            let segments: HashSet<char> = digit.chars().collect();
            if let Some(x) = dict.get_mut(&n) {
                let new: HashSet<char> = x.intersection(&segments).cloned().collect();
                *x = new;
            } else {
                dict.insert(n, segments);
            }
        }
        let mut table = vec!['.'; 7];
        table[0] = *(&dict[&3] - &dict[&2]).iter().next().unwrap();
        table[6] = *(&(&dict[&5] & &dict[&6]) - &HashSet::from([table[0]]))
            .iter()
            .next()
            .unwrap();
        table[3] = *(&dict[&5] - &HashSet::from([table[0], table[6]]))
            .iter()
            .next()
            .unwrap();
        table[5] = *(&dict[&6] & &dict[&2]).iter().next().unwrap();
        table[2] = *(&dict[&3] - &HashSet::from([table[0], table[5]]))
            .iter()
            .next()
            .unwrap();
        table[1] = *(&dict[&6] - &HashSet::from([table[0], table[5], table[6]]))
            .iter()
            .next()
            .unwrap();
        table[4] = *(&(&dict[&7] - &dict[&6]) - &HashSet::from([table[2], table[3]]))
            .iter()
            .next()
            .unwrap();
        let trans_digits: HashMap<String, _> = digits
            .iter()
            .map(|(repr, val)| {
                (
                    repr.chars()
                        .map(|c| table[(c as usize - '0' as usize)])
                        .join("")
                        .chars()
                        .sorted()
                        .join(""),
                    val,
                )
            })
            .collect();
        let value = number
            .split_ascii_whitespace()
            .map(|digit| trans_digits[&digit.chars().sorted().join("")])
            .fold(0, |total, digit| total * 10 + digit);
        s += value;
    }
    println!("{}", s);
}

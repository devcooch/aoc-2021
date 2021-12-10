use std::collections::HashMap;

fn main() {
    let score = HashMap::from([(')', 1usize), (']', 2), ('}', 3), ('>', 4)]);
    let pair = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let rpair = pair
        .iter()
        .map(|(key, value)| (value, key))
        .collect::<HashMap<_, _>>();
    let data = include_str!("day10.txt");
    let mut result: Vec<_> = data
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(c),
                    ')' | ']' | '}' | '>' => {
                        if pair[&c] != stack.pop().unwrap() {
                            return Err(c);
                        }
                    }
                    _ => panic!("Unknown character <{}>", c),
                }
            }
            Ok(stack)
        })
        .filter_map(|x| x.ok())
        .map(|add| add.iter().rev().fold(0, |t, x| t * 5 + score[rpair[x]]))
        .collect();
    result.sort_unstable();
    println!("{}", result[result.len() / 2]);
}

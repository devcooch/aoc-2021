use std::collections::HashMap;

fn main() {
    let score = HashMap::from([(')', 3usize), (']', 57), ('}', 1197), ('>', 25137)]);
    let pair = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let data = include_str!("day10.txt");
    let result: usize = data
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            let mut line_score = 0;
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(c),
                    ')' | ']' | '}' | '>' => {
                        if pair[&c] != stack.pop().unwrap() {
                            line_score = score[&c];
                            break;
                        }
                    }
                    _ => panic!("Unknown character <{}>", c),
                }
            }
            line_score
        })
        .sum();
    println!("{}", result);
}

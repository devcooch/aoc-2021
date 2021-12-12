use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Size {
    Small,
    Big,
}

fn traverse(start: usize, end: usize, g: &[Vec<usize>], v: &mut Vec<usize>, s: &[Size]) -> usize {
    if start == end {
        return 1;
    }
    let mut result = 0;
    if s[start] == Size::Big || v[start] == 0 {
        v[start] += 1;
        for target in g[start].iter() {
            result += traverse(*target, end, g, v, s);
        }
        v[start] -= 1;
    }
    result
}

fn main() {
    let data = include_str!("day12.txt");
    let mut g: Vec<Vec<usize>> = Vec::new();
    let mut names: HashMap<String, usize> = HashMap::new();
    let mut sizes = Vec::new();
    for line in data.lines() {
        let (from_n, to_n) = line
            .split('-')
            .map(|name| name.to_string())
            .map(|name| {
                let n = names.len();
                let size = if name.chars().all(|c| c.is_uppercase()) {
                    Size::Big
                } else {
                    Size::Small
                };
                let res = *names.entry(name).or_insert(n);
                if g.len() <= res {
                    g.push(Vec::new());
                    assert!(g.len() == res + 1);
                    sizes.push(size);
                    assert!(sizes.len() == res + 1);
                }
                res
            })
            .collect_tuple()
            .unwrap();
        g[from_n].push(to_n);
        g[to_n].push(from_n);
    }
    let start = names.get("start").unwrap();
    let end = names.get("end").unwrap();
    let mut visited: Vec<usize> = vec![0; names.len()];
    let count = traverse(*start, *end, &g, &mut visited, &sizes);
    println!("{}", count);
}

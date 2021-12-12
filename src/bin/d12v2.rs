use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Size {
    Small,
    Big,
}

fn traverse(
    start: usize,
    end: usize,
    g: &[Vec<usize>],
    s: &[Size],
    v: &mut Vec<usize>,
    twice: &mut Option<usize>,
) -> usize {
    if start == end {
        return 1;
    }
    let mut result = 0;
    if s[start] == Size::Big || v[start] == 0 || twice.is_none() {
        v[start] += 1;
        if s[start] == Size::Small && v[start] == 2 && twice.is_none() {
            *twice = Some(start);
        }
        for target in g[start].iter() {
            result += traverse(*target, end, g, s, v, twice);
        }
        v[start] -= 1;
        if *twice == Some(start) {
            *twice = None;
        }
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
    for t in g[*start].to_vec() {
        // prohibit coming back to start
        g[t].retain(|&x| x != *start);
    }
    g[*end].clear();
    let mut visited: Vec<usize> = vec![0; names.len()];
    let mut twice: Option<usize> = None;
    let count = traverse(*start, *end, &g, &sizes, &mut visited, &mut twice);
    println!("{}", count);
}

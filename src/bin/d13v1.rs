use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    type PointSet = HashSet<(usize, usize)>;
    let data = include_str!("day13.txt");
    let mut lines = data.lines();
    let mut points: PointSet = PointSet::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        points.insert(
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap(),
        );
    }
    let mut folds: Vec<(char, usize)> = Vec::new();
    for line in lines {
        let (fold_split, at) = line.split_once('=').unwrap();
        folds.push((
            fold_split.chars().last().unwrap(),
            at.parse::<usize>().unwrap(),
        ));
    }
    let (along, at) = folds[0];
    let (fold, keep): (PointSet, PointSet) = match along {
        'x' => points.iter().partition(|&p| p.0 > at),
        'y' => points.iter().partition(|&p| p.1 > at),
        _ => panic!(""),
    };
    points = keep;
    match along {
        'x' => fold.iter().map(|p| (at - (p.0 - at), p.1)).for_each(|p| {
            points.insert(p);
        }),
        'y' => fold.iter().map(|p| (p.0, at - (p.1 - at))).for_each(|p| {
            points.insert(p);
        }),
        _ => panic!(""),
    }
    println!("{}", points.len());
}

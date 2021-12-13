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
    for (along, at) in folds {
        match along {
            'x' => {
                let (fold, keep): (PointSet, PointSet) = points.iter().partition(|&p| p.0 > at);
                points = keep;
                fold.iter().map(|p| (at - (p.0 - at), p.1)).for_each(|p| {
                    points.insert(p);
                });
            }
            'y' => {
                let (fold, keep): (PointSet, PointSet) = points.iter().partition(|&p| p.1 > at);

                points = keep;
                fold.iter().map(|p| (p.0, at - (p.1 - at))).for_each(|p| {
                    points.insert(p);
                });
            }
            _ => panic!("Unknown axis {}", along),
        }
    }
    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();
    let mut m = vec![vec!['.'; max_x + 1]; max_y + 1];
    points.iter().for_each(|p| m[p.1][p.0] = '#');
    println!("{}", m.iter().map(|r| r.iter().join("")).join("\n"));
}

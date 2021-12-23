use itertools::Itertools;
use std::ops::RangeInclusive;

type MyRange = RangeInclusive<i64>;

#[derive(Debug)]
struct Cuboid {
    action: usize,
    x: MyRange,
    y: MyRange,
    z: MyRange,
}

impl Cuboid {
    fn new(action: usize, (x, y, z): (MyRange, MyRange, MyRange)) -> Cuboid {
        Cuboid { action, x, y, z }
    }
}

fn main() {
    let data = include_str!("day22.txt");
    let cuboids = data
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            let a = (left == "on") as usize;
            let xyz = right
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(|r| {
                    let (ss, es) = r.split_once('=').unwrap().1.split_once("..").unwrap();
                    let s = ss.parse::<i64>().unwrap() + 50;
                    let e = es.parse::<i64>().unwrap() + 50;
                    MyRange::new(s, e)
                })
                .collect_tuple()
                .unwrap();
            Cuboid::new(a, xyz)
        })
        .collect::<Vec<_>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert!(true);
    }
}

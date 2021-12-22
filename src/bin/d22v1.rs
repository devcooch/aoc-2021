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
    let mut m = vec![vec![vec![0; 101]; 101]; 101];
    cuboids
        .iter()
        .filter(|&c| {
            *c.x.start() >= 0
                && *c.x.end() <= 100
                && *c.y.start() >= 0
                && *c.y.end() <= 100
                && *c.z.start() >= 0
                && *c.z.end() <= 100
        })
        .for_each(|c| {
            for x in c.x.clone() {
                for y in c.y.clone() {
                    for z in c.z.clone() {
                        m[x as usize][y as usize][z as usize] = c.action;
                    }
                }
            }
        });
    let total: usize = m
        .iter()
        .map(|x| x.iter().map(|y| y.iter().sum::<usize>()).sum::<usize>())
        .sum();
    println!("{}", total);
}

use itertools::Itertools;
use std::cmp::{max, min, Ordering};
use std::fmt;
use std::ops::RangeInclusive;

type MyRange = RangeInclusive<i64>;

#[derive(Debug)]
struct Cuboid {
    action: usize,
    x: MyRange,
    y: MyRange,
    z: MyRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cluster {
    x: MyRange,
    y: MyRange,
    z: MyRange,
}

impl Cuboid {
    fn new(action: usize, (x, y, z): (MyRange, MyRange, MyRange)) -> Cuboid {
        Cuboid { action, x, y, z }
    }
}

impl Cluster {
    fn new(c: &Cuboid) -> Cluster {
        Cluster {
            x: c.x.clone(),
            y: c.y.clone(),
            z: c.z.clone(),
        }
    }

    fn can_merge(&self, other: &Cluster) -> bool {
        [other.x.start(), other.x.end()]
            .iter()
            .cartesian_product([other.y.start(), other.y.end()])
            .cartesian_product([other.z.start(), other.z.end()])
            .any(|((x, y), z)| {
                *x >= self.x.start()
                    && *x <= self.x.end()
                    && y >= self.y.start()
                    && y <= self.y.end()
                    && z >= self.z.start()
                    && z <= self.z.end()
            })
    }

    fn merge(&mut self, other: &Cluster) {
        self.x = min(*self.x.start(), *other.x.start())..=max(*self.x.end(), *other.x.end());
        self.y = min(*self.y.start(), *other.y.start())..=max(*self.y.end(), *other.y.end());
        self.z = min(*self.z.start(), *other.z.start())..=max(*self.z.end(), *other.z.end());
    }

    fn size(&self) -> usize {
        assert!(self.x.end() > self.x.start());
        assert!(self.y.end() > self.y.start());
        assert!(self.z.end() > self.z.start());
        ((self.x.end() - self.x.start()) as usize + 1)
            * ((self.y.end() - self.y.start()) as usize + 1)
            * ((self.z.end() - self.z.start()) as usize + 1)
    }
}

impl fmt::Display for Cluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cluster({}..{}, {}..{}, {}..{})",
            self.x.start(),
            self.x.end(),
            self.y.start(),
            self.y.end(),
            self.z.start(),
            self.z.end()
        )
    }
}

impl Ord for Cluster {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x.start(), self.y.start(), self.z.start()).cmp(&(
            other.x.start(),
            other.y.start(),
            other.y.end(),
        ))
    }
}

impl PartialOrd for Cluster {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    let mut clusters: Vec<Cluster> = cuboids
        .iter()
        .filter(|&c| c.action > 0)
        .map(|c| Cluster::new(c))
        .collect();
    clusters.sort();
    println!(
        "Starting with {} cuboids and {} clusters",
        cuboids.len(),
        clusters.len()
    );
    let mut taken = vec![false; clusters.len()];
    let mut merged = Vec::new();
    merged.push(clusters.iter().next().unwrap().clone());
    taken[0] = true;
    while let Some((i, _)) = taken.iter().enumerate().find(|(_, &x)| !x) {
        merged.push(clusters[i].clone());
        taken[i] = true;
        if let Some(last) = merged.last_mut() {
            for (j, c) in clusters.iter().enumerate() {
                if taken[j] {
                    continue;
                }
                if last.can_merge(&c) {
                    last.merge(&c);
                    taken[j] = true;
                }
            }
        }
    }
    for c in merged.iter() {
        println!("{}", c);
    }
    println!(
        "Merged to {} clusters with total size {}",
        merged.len(),
        merged.iter().map(|c| c.size()).sum::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_merge() {
        let c1 = Cluster {
            x: 0..=10,
            y: 0..=10,
            z: 0..=10,
        };
        let c2 = Cluster {
            x: 11..=20,
            y: 11..=20,
            z: 5..=20,
        };
        let c3 = Cluster {
            x: 5..=15,
            y: 5..=15,
            z: 9..=12,
        };
        assert!(!c1.can_merge(&c2));
        assert!(c1.can_merge(&c3));
        assert!(c2.can_merge(&c3));
    }

    #[test]
    fn test_merge() {
        let mut c1 = Cluster {
            x: 0..=10,
            y: 0..=10,
            z: 0..=10,
        };
        let c2 = Cluster {
            x: 11..=20,
            y: 11..=20,
            z: 5..=20,
        };
        let c3 = Cluster {
            x: 5..=15,
            y: 5..=15,
            z: 9..=12,
        };
        c1.merge(&c3);
        assert!(c1.can_merge(&c2));
        c1.merge(&c2);
        assert_eq!(
            c1,
            Cluster {
                x: 0..=20,
                y: 0..=20,
                z: 0..=20,
            },
        );
        assert_eq!(c1.size(), 21 * 21 * 21);
    }
}

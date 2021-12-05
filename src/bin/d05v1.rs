use std::cmp::{max, min};

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Vent {
    start: Coords,
    end: Coords,
}

fn get_coords(s: &str) -> Coords {
    let mut i = s.split(',').map(|x| x.parse::<usize>().unwrap());
    Coords {
        x: i.next().unwrap(),
        y: i.next().unwrap(),
    }
}

fn main() {
    let data = include_str!("day05.txt");
    let mut vents = Vec::<Vent>::new();
    for line in data.lines() {
        let mut coords = line.split(" -> ").map(|x| get_coords(x));
        let vent = Vent {
            start: coords.next().unwrap(),
            end: coords.next().unwrap(),
        };
        if (vent.start.x == vent.end.x) || (vent.start.y == vent.end.y) {
            vents.push(vent);
        }
    }
    let start_max_x = vents.iter().map(|v| v.start.x).max().unwrap();
    let end_max_x = vents.iter().map(|v| v.end.x).max().unwrap();
    let max_x = max(start_max_x, end_max_x) + 1;
    let start_max_y = vents.iter().map(|v| v.start.y).max().unwrap();
    let end_max_y = vents.iter().map(|v| v.end.y).max().unwrap();
    let max_y = max(start_max_y, end_max_y) + 1;
    let mut field = vec![vec![0; max_x]; max_y];
    for vent in vents {
        if vent.start.x == vent.end.x {
            let from = min(vent.start.y, vent.end.y);
            let to = max(vent.start.y, vent.end.y);
            for i in from..to + 1 {
                field[i][vent.start.x] += 1;
            }
        } else {
            let from = min(vent.start.x, vent.end.x);
            let to = max(vent.start.x, vent.end.x);
            for i in from..to + 1 {
                field[vent.start.y][i] += 1;
            }
        }
    }
    println!(
        "{}",
        field
            .iter()
            .map(|row| row.iter().filter(|&x| x > &1).count())
            .sum::<usize>()
    );
}

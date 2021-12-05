use std::cmp::max;

#[derive(Debug)]
struct Coords {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Vent {
    start: Coords,
    end: Coords,
}

fn get_coords(s: &str) -> Coords {
    let mut i = s.split(',').map(|x| x.parse::<i64>().unwrap());
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
        vents.push(Vent {
            start: coords.next().unwrap(),
            end: coords.next().unwrap(),
        });
    }
    let start_max_x = vents.iter().map(|v| v.start.x).max().unwrap() as usize;
    let end_max_x = vents.iter().map(|v| v.end.x).max().unwrap() as usize;
    let max_x = max(start_max_x, end_max_x) + 1;
    let start_max_y = vents.iter().map(|v| v.start.y).max().unwrap() as usize;
    let end_max_y = vents.iter().map(|v| v.end.y).max().unwrap() as usize;
    let max_y = max(start_max_y, end_max_y) + 1;
    let mut field = vec![vec![0usize; max_x]; max_y];
    for vent in vents {
        let mut x = vent.start.x;
        let mut y = vent.start.y;
        let dx = (vent.end.x - vent.start.x).signum();
        let dy = (vent.end.y - vent.start.y).signum();
        while x != vent.end.x || y != vent.end.y {
            field[y as usize][x as usize] += 1;
            x += dx;
            y += dy;
        }
        field[y as usize][x as usize] += 1;
    }
    println!(
        "{}",
        field
            .iter()
            .map(|row| row.iter().filter(|&x| x > &1).count())
            .sum::<usize>()
    );
}

use std::collections::HashMap;

#[derive(Debug)]
struct RowCol {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Board {
    columns_fill: Vec<usize>,
    rows_fill: Vec<usize>,
    values: HashMap<usize, RowCol>,
}

impl Board {
    fn new() -> Self {
        Board {
            columns_fill: vec![0; 5],
            rows_fill: vec![0; 5],
            values: HashMap::new(),
        }
    }
    fn won(&self) -> bool {
        self.columns_fill.iter().any(|&x| x == 5) || self.rows_fill.iter().any(|&x| x == 5)
    }
    fn sum_left_values(&self) -> usize {
        self.values.keys().sum()
    }
    fn check_value(&mut self, value: usize) {
        if let Some(x) = self.values.remove(&value) {
            self.columns_fill[x.col] += 1;
            self.rows_fill[x.row] += 1;
        }
    }
}

fn ignore<T>(_: Option<T>) {}

fn main() {
    let data = include_str!("day04.txt");
    let mut lines = data.lines();
    let serie = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut boards: Vec<Board> = Vec::new();
    while let Some(_) = lines.next() {
        let mut b = Board::new();
        for row in 0..5 {
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .enumerate()
                .for_each(|(col, value)| ignore(b.values.insert(value, RowCol { row, col })));
        }
        assert!(b.values.len() == 25);
        boards.push(b);
    }
    let mut all_winners = Vec::<(Board, usize)>::new();
    for number in serie {
        boards.iter_mut().for_each(|b| b.check_value(number));
        let winners = boards
            .iter()
            .enumerate()
            .filter(|&(_, b)| b.won())
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        for i in winners.iter().rev() {
            let board = boards.remove(*i);
            all_winners.push((board, number));
        }
    }
    let (board, number) = all_winners.last().unwrap();
    println!("{}", number * board.sum_left_values());
}

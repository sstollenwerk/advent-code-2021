use crate::lib::to_filename;
use crate::lib::transpose;

use std::fs;

type Matrix<T> = Vec<Vec<T>>;

type Bingo = Matrix<(i32, bool)>;

fn get_data() -> (Vec<i32>, Vec<Bingo>) {
    let rows = fs::read_to_string(to_filename(4))
        .expect("Could not read file")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let head = read_csv_row(&rows[0]);

    let boxes = (rows[1..].chunks(6))
        .map(|b| read_box(b.to_vec()))
        .collect();

    (head, boxes)
}

fn read_csv_row(s: &str) -> Vec<i32> {
    s.split(',')
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn read_white_row(s: &String) -> Vec<i32> {
    s.split_whitespace()
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn read_box(rect: Vec<String>) -> Bingo {
    let mut rect = rect.clone();
    rect.retain(|x| x.len() > 0);

    rect.iter().map(read_white_row).map(to_bingo).collect()
}

fn to_bingo(bin: Vec<i32>) -> Vec<(i32, bool)> {
    bin.iter().map(|c| (*c, false)).collect()
}

fn draw(mut square: Bingo, n: i32) -> Bingo {
    for i in (0..square.len()) {
        for j in (0..square[0].len()) {
            let cell = square[i].get_mut(j).unwrap();
            if cell.0 == n {
                *cell = (n, true);
            }
        }
    }

    square
}

fn vic(b: &Bingo) -> bool {
    for r in b {
        if r.iter().map(|x| x.1).all(|x| x) {
            return true;
        }
    }
    false
}

fn victory(b: &Bingo) -> bool {
    vic(b) | vic(&transpose(b))
}

fn score(b: &Bingo, c: i32) -> i32 {
    let s: i32 = b.iter().flatten().filter(|x| !x.1).map(|x| x.0).sum();
    c * (s)
}

pub fn part1() -> i32 {
    let (calls, boxes) = get_data();

    let mut boxes = boxes;

    for c in calls.iter() {
        boxes = boxes.iter().map(|b| draw(b.to_vec(), *c)).collect();

        let mut wins = boxes.iter().filter(|b| victory(b));
        let w = wins.next();

        if w.is_some() {
            let win = w.unwrap();
            println!("{:?}", win);
            return score(win, *c);
        }
    }

    19
}
pub fn part2() -> i32 {
    let (calls, boxes) = get_data();

    let mut boxes = boxes;

    for c in calls.iter() {
        boxes = boxes.iter().map(|b| draw(b.to_vec(), *c)).collect();
        if boxes.len() > 1 {
            boxes.retain(|b| !victory(b));
        }
        if boxes.len() == 1 {
            let poss = &boxes[0];
            if victory(poss) {
                println!("{:?}", poss);
                return score(poss, *c);
            }
        }
    }

    19
}

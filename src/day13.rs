use crate::lib::to_filename;

use std::fs;

use std::collections::HashSet;

use itertools::{Either, Itertools};

use num_complex::Complex;

type Position = i32;

type Place = Complex<Position>;

#[derive(Debug, Clone, Copy)]
enum Fold {
    X(Position),
    Y(Position),
}

type Grid = HashSet<Place>;

type Res = u32;

fn get_data() -> (Grid, Vec<Fold>) {
    let (grid_, folds): (Vec<_>, Vec<_>) = fs::read_to_string(to_filename(13))
        .expect("Could not read file")
        .lines()
        .filter(|s| s.len() > 0)
        .map(|s| read_row(s))
        .partition_map(
            |r| r, //          match r {
                   //         itertools::Either::Left(v) => itertools::Either::Left(v),
                   //          itertools::Either::Right(v) => itertools::Either::Right(v),
                   // }
        );

    let grid: Grid = grid_.into_iter().collect();
    (grid, folds)
}

fn read_row(row: &str) -> Either<Place, Fold> {
    if row.contains(',') {
        let data = row
            .split(',')
            .map(|s| s.parse::<Position>().unwrap())
            .collect::<Vec<_>>();
        return itertools::Either::Left(Complex::new(data[0], data[1]));
    }

    let pos = row.split('=').collect::<Vec<_>>()[1]
        .parse::<Position>()
        .unwrap();
    if row.contains('y') {
        return itertools::Either::Right(Fold::Y(pos));
    } else if row.contains('x') {
        return itertools::Either::Right(Fold::X(pos));
    } else {
        panic!()
    }
}

fn new_pos(n: Position, fold: Position) -> Position {
    if n < fold {
        return n;
    } else {
        return (2 * fold) - n;
    }
}

fn translocate(p: Place, f: Fold) -> Place {
    match f {
        Fold::Y(a) => Complex::new(p.re, new_pos(p.im, a)),
        Fold::X(a) => Complex::new(new_pos(p.re, a), p.im),
    }
}

fn make_fold(grid: Grid, f: Fold) -> Grid {
    let mut res = HashSet::new();
    for p in grid.into_iter() {
        let k = translocate(p, f);
        //println!("{:?}", (p,k,f)  );

        res.insert(k);
    }
    res
}

fn display(grid: Grid) -> () {
    let vals = grid
        .into_iter()
        .map(|c| (c.re as usize, c.im as usize))
        .collect::<Vec<_>>();
    let y_ = vals.iter().map(|t| t.0).max().unwrap();
    let x_ = vals.iter().map(|t| t.1).max().unwrap();

    let mut row = Vec::new();
    for _ in (0..=y_) {
        row.push(' ')
    }

    let mut rows = Vec::new();
    for _ in (0..=x_) {
        rows.push(row.clone())
    }
    for (a, b) in vals.into_iter() {
        rows[b][a] = '*'
    }

    for r in rows.iter() {
        println!("{:?}", r);
    }
}

pub fn part1() -> Res {
    let (grid, folds) = get_data();
    println!("{:?}", grid);
    println!("{:?}", grid.len());
    let res = make_fold(grid, folds[0]);
    println!("{:?}", res);
    res.len() as Res
}
pub fn part2() -> Res {
    let (grid_, folds) = get_data();
    let mut grid = grid_;
    for f in folds.into_iter() {
        grid = make_fold(grid, f);
    }
    println!("{:?}", grid);
    display(grid);
    0
}

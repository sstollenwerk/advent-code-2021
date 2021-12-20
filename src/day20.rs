use crate::lib::to_filename;
use crate::lib::transpose;

use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use cached::proc_macro::cached;
use num_complex::Complex;

type Num = i32;

type Place = Complex<Num>;

type Alg = Vec<bool>;

type Grid = HashSet<Place>;
type HashGrid = Vec<(Place, bool)>;

const BOUNDARY: Num = 8;

fn get_data() -> (Alg, Grid) {
    let mut grid = HashSet::new();

    let rows_: Vec<_> = fs::read_to_string(to_filename(20))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect();

    let mut rows = rows_.iter();

    let alg = rows.next().unwrap().to_vec();
    rows.next();
    for (i, row) in rows.enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c {
                grid.insert(Complex::new(i as i32, j as i32));
            }
        }
    }

    (alg, grid)
}

fn read_row(row: &str) -> Vec<bool> {
    row.chars()
        .map(|d| match d {
            '#' => true,
            '.' => false,
            _ => panic!(),
        })
        .collect::<Vec<_>>()
}

fn get_to_check(g: &Grid) -> Vec<Place> {
    let mut res = Vec::new();

    let left = g.iter().map(|c| c.re).min().unwrap();
    let right = g.iter().map(|c| c.re).max().unwrap();

    let top = g.iter().map(|c| c.im).min().unwrap();
    let bottom = g.iter().map(|c| c.im).max().unwrap();
    for i in (left - BOUNDARY..=right + BOUNDARY) {
        for j in (top - BOUNDARY..=bottom + BOUNDARY) {
            res.push(Place::new(i, j))
        }
    }
    res
}

fn as_base(data: &Vec<Num>, base: Num) -> Num {
    //  println!("{:?}", ( data, base) );
    let mut res = 0;
    for i in data {
        res *= base;
        res += *i;
    }
    res
}

fn neigbours(n: Place, g: &Grid) -> Num {
    let mut res = Vec::new();
    for a in (-1..=1) {
        for b in (-1..=1) {
            let k = Place::new(a, b);
            res.push(g.contains(&(n + k)) as Num)
        }
    }
    as_base(&res, 2)
}

fn step(alg: &Alg, g: &Grid) -> Grid {
    let mut grid = HashSet::new();

    for n in get_to_check(g) {
        if alg[neigbours(n, g) as usize] {
            grid.insert(n);
        }
    }

    grid
}

fn top_left(grid: &Grid) -> Place {
    let left = grid.iter().map(|c| c.re).min().unwrap();
    let top = grid.iter().map(|c| c.im).min().unwrap();
    Complex::new(left, top)
}

fn bottom_right(grid: &Grid) -> Place {
    let a = grid.iter().map(|c| c.re).max().unwrap();
    let b = grid.iter().map(|c| c.im).max().unwrap();
    Complex::new(a, b)
}

fn l_inf(c: Complex<Num>) -> Num {
    // https://en.wikipedia.org/wiki/L-infinity
    let vals = vec![c.re, c.im];
    vals.iter().map(|n| n.abs()).max().unwrap()
}

fn lowest_dist(c: Complex<Num>) -> Num {
    let vals = vec![c.re, c.im];
    vals.iter().map(|n| n.abs()).min().unwrap()
}

fn clear(grid: &Grid, alg: &Alg) -> Grid {
    let dist = BOUNDARY + 2;
    let mut res = Grid::new();
    let a = top_left(grid);
    let b = bottom_right(grid);
    if !(alg[0] && grid.contains(&a) && grid.contains(&b)) {
        res = grid.clone();
    } else {
        for c in grid.iter() {
            if lowest_dist(a - *c) > dist && lowest_dist(b - *c) > dist {
                res.insert(*c);
            }
        }
    }
    res
}

fn display(grid: &Grid) -> () {
    let left = grid.iter().map(|c| c.re).min().unwrap();
    let top = grid.iter().map(|c| c.im).min().unwrap();
    let k = Complex::new(left, top);
    let vals = grid
        .into_iter()
        .map(|c| c - k)
        .map(|c| (c.re as usize, c.im as usize))
        .collect::<Vec<_>>();
    let y_ = vals.iter().map(|t| t.0).max().unwrap();
    let x_ = vals.iter().map(|t| t.1).max().unwrap();

    let mut row = Vec::new();
    for _ in (0..=y_) {
        row.push('.')
    }

    let mut rows = Vec::new();
    for _ in (0..=x_) {
        rows.push(row.clone())
    }
    for (a, b) in vals.into_iter() {
        rows[b][a] = '#'
    }

    rows = transpose(&rows);

    for r in rows.iter() {
        println!("{:?}", r.into_iter().collect::<String>());
    }
}

pub fn part1() -> Num {
    let (alg, nodes_) = get_data();

    let mut nodes = nodes_;
    println!("{:?}", nodes);

    for i in (0..2) {
        println!("{:?}", i);

        nodes = step(&alg, &nodes);
    }
    nodes = clear(&nodes, &alg);


    //  display(nodes);

    nodes.len() as Num
}
pub fn part2() -> Num {
    // todo!();
    let (alg, nodes_) = get_data();

    let mut nodes = nodes_;
  //  println!("{:?}", nodes);

    for i in (0..50) {
        println!("{:?}", i);

        nodes = step(&alg, &nodes);
        if i % 2 == 1 {
            nodes = clear(&nodes, &alg);
            //  display(&nodes);
        }
    }
    //  display(nodes);

    nodes.len() as Num
}

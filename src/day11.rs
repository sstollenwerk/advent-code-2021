use crate::lib::to_filename;

use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

use num_complex::Complex;

type Place = Complex<i32>;

type Energy = u32;
type Result = u32;

type Grid = HashMap<Place, Energy>;

fn get_data() -> Grid {
    let mut grid = HashMap::new();

    for (i, row) in fs::read_to_string(to_filename(11))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .enumerate()
    {
        for (j, c) in row.iter().enumerate() {
            grid.insert(Complex::new(i as i32, j as i32), (*c as Energy));
        }
    }

    grid
}

fn read_row(row: &str) -> Vec<u32> {
    row.chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
}

fn adjacent_places(g: &Grid, pos: Place) -> Vec<Place> {
    let mut nears = Vec::new();
    let dirs = [-1, 0, 1];
    for i in dirs.iter() {
        for j in dirs.iter() {
            nears.push(Complex::new(*i, *j))
        }
    }
    nears
        .iter()
        .map(|c| pos + c)
        .filter(|c| g.contains_key(&c))
        .collect()
}

fn substep(g: Grid, to_check: HashSet<Place>) -> (Grid, HashSet<Place>) {
    let mut grid = g.clone();

    let mut flashed: HashSet<Place> = HashSet::new();
    let mut adjacents = Vec::new();
    for i in to_check.into_iter() {
        if grid.get(&i).unwrap() > &9 {
            flashed.insert(i);
            adjacents.append(&mut adjacent_places(&g, i));
        }
    }
    for n in adjacents.iter() {
        *grid.get_mut(&n).unwrap() += 1;
    }

    (grid, flashed)
}

fn step(g: &Grid) -> (Grid, Result) {
    let mut grid = g.clone();

    for (_, val) in grid.iter_mut() {
        *val += 1;
    }

    let mut flashed: HashSet<Place> = HashSet::new();
    let keys_ = g.keys().map(|x| *x).collect::<HashSet<Place>>();
    loop {
        let to_check: HashSet<Place> = keys_.difference(&flashed).map(|x| *x).collect();
        let (grid_, new_flashed) = substep(grid, to_check);
        grid = grid_;
        if new_flashed.is_empty() {
            break;
        }
        flashed = flashed
            .union(&new_flashed)
            .map(|x| *x)
            .collect::<HashSet<Place>>();
    }

    for n in flashed.iter() {
        *grid.get_mut(&n).unwrap() = 0;
    }

    (grid, flashed.len() as Result)
}

pub fn part1() -> Result {
    let mut grid = get_data();
    let mut res: Result = 0;
    for _ in (0..100) {
        let (grid_, flashed) = step(&grid);
        grid = grid_;
        res += flashed;
    }

    res
}

pub fn part2() -> Result {
    let mut grid = get_data();
    let mut iters: Result = 0;
    loop {
        iters += 1;
        let (grid_, flashed) = step(&grid);
        grid = grid_;
        if flashed == (grid.len() as Result) {
            break;
        }
    }
    iters
}

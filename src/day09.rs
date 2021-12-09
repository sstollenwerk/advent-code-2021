use crate::lib::to_filename;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

use num_complex::Complex;

type Height = u32;

type Place = Complex<i32>;

type Grid = HashMap<Place, Height>;

fn get_data() -> Grid {
    let mut grid = HashMap::new();

    for (i, row) in fs::read_to_string(to_filename(9))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .enumerate()
    {
        for (j, c) in row.iter().enumerate() {
            grid.insert(Complex::new(i as i32, j as i32), (*c as Height));
        }
    }

    grid
}

fn read_row(row: &str) -> Vec<u32> {
    row.chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
}

fn adjacent(g: &Grid, pos: Place) -> Vec<Height> {
    let nears = [
        Complex::new(1, 0),
        Complex::new(-1, 0),
        Complex::new(0, 1),
        Complex::new(0, -1),
    ];
    nears
        .iter()
        .map(|c| pos + c)
        .filter_map(|c| g.get(&c))
        .map(|c| *c)
        .collect()
}
pub fn part1() -> Height {
    let grid = get_data();
    println!("{:?}", grid);

    let lows: Vec<Height> = grid
        .keys()
        .filter(|c| grid.get(c).unwrap() < adjacent(&grid, **c).iter().min().unwrap())
        .map(|c| *grid.get(c).unwrap())
        .collect();
    lows.iter().map(|x| x + 1).sum()
}

pub fn part2() -> Height {
    0
}

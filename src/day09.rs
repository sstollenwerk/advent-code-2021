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

fn adjacent_places(g: &Grid, pos: Place) -> Vec<Place> {
    let nears = [
        Complex::new(1, 0),
        Complex::new(-1, 0),
        Complex::new(0, 1),
        Complex::new(0, -1),
    ];
    nears
        .iter()
        .map(|c| pos + c)
        .filter(|c| g.contains_key(&c))
        .collect()
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

fn build_basin(g: &Grid, p: Place) -> HashSet<Place> {
    let mut basin: HashSet<Place> = HashSet::new();

    let mut to_check: HashSet<Place> = HashSet::new();
    to_check.insert(p);

    while !to_check.is_empty() {
        let c = *to_check.iter().next().unwrap();
        to_check.remove(&c);
        basin.insert(c);
        let nexts = adjacent_places(g, c)
            .iter()
            .map(|c| *c)
            .filter(|c| *g.get(&c).unwrap() != 9)
            .collect::<HashSet<Place>>()
            .difference(&basin)
            .map(|c| *c)
            .collect::<HashSet<Place>>();
        for n in nexts.iter() {
            to_check.insert(*n);
        }
    }

    basin
}

fn basins(g: &Grid) -> Vec<HashSet<Place>> {
    let mut seen: HashSet<Place> = HashSet::new();
    let mut groups = Vec::new();

    for (k, v) in g.iter() {
        let already_in = !seen.insert(*k);

        if (already_in || *v == 9) {
            continue;
        }

        let basin = build_basin(g, *k);
        groups.push(basin.clone());
        for s in basin.iter() {
            seen.insert(*s);
        }
    }
    groups
}

fn largests<T: Clone + std::cmp::Ord>(vals: &Vec<T>, n: u32) -> Vec<T> {
    let mut v = vals.clone();
    v.sort();
    v.reverse();
    v[..n as usize].to_vec()
}

pub fn part1() -> Height {
    let grid = get_data();

    let lows: Vec<Height> = grid
        .keys()
        .filter(|c| grid.get(c).unwrap() < adjacent(&grid, **c).iter().min().unwrap())
        .map(|c| *grid.get(c).unwrap())
        .collect();
    lows.iter().map(|x| x + 1).sum()
}

pub fn part2() -> Height {
    let basins = basins(&get_data());

    let sizes = basins
        .iter()
        .map(|h| h.iter().count())
        .collect::<Vec<usize>>();
    largests(&sizes, 3).iter().product::<usize>() as Height
}

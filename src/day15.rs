use crate::lib::to_filename;

use std::fs;

use std::collections::HashMap;

use cached::proc_macro::cached;
use num_complex::Complex;

type Place = Complex<i32>;

type Cost = u32;

type Grid = HashMap<Place, Cost>;
type HashGrid = Vec<(Place, Cost)>;

fn get_data() -> Grid {
    let mut grid = HashMap::new();

    for (i, row) in fs::read_to_string(to_filename(15))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .enumerate()
    {
        for (j, c) in row.iter().enumerate() {
            grid.insert(Complex::new(i as i32, j as i32), (*c as Cost));
        }
    }

    grid
}

fn read_row(row: &str) -> Vec<Cost> {
    row.chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
}

fn get_goal(g: &Grid) -> Place {
    let p = g.keys().map(|c| (c.re, c.im)).max().unwrap();
    Complex::new(p.0, p.1)
}
fn as_grid(g: &HashGrid) -> Grid {
    g.into_iter().map(|c| *c).collect()
}

fn as_hashable(g: &Grid) -> HashGrid {
    g.into_iter().map(|c| (*c.0, *c.1)).collect()
}
fn lowest_cost(g: Grid, start: Place, end: Place) -> Cost {
    let mut places: Vec<Place> = g.keys().map(|c| *c).collect();
    places.sort_by_key(|c| (c.re, c.im));
    places.reverse();

    let mut costs: HashMap<Place, Cost> = HashMap::new();
    costs.insert(end, 0);

    let dirs = vec![Complex::new(0, 1), Complex::new(1, 0)];

    println!("{:?}", places);

    for p in places.into_iter() {
        if costs.contains_key(&p) {
            continue;
        }
        let nexts = dirs.iter().map(|c| *c + p).filter(|c| g.contains_key(&c));

        //     println!("{:?}",costs);
        //       println!("{:?}",nexts);
        //         println!("{:?}",p);

        let res = nexts.map(|c| g[&c] + costs[&c]).min().unwrap();

        costs.insert(p, res);
    }
    costs[&start]
}

/*
#[cached]
fn lowest_cost(g_: HashGrid, start: Place, end: Place) -> Option<Cost> {
    if start == end {
        return Some(0);
    }

    let g = as_grid(g_);

    let dirs = vec![Complex::new(0, 1), Complex::new(1, 0)];
    let nexts = dirs
        .into_iter()
        .map(|c| c + start)
        .filter(|c| g.contains_key(&c));

    nexts
        .into_iter()
        .map(|c| g[&c] + lowest_cost(g_, c, end).unwrap())
        .min()
}
*/
// couldn't get it to work
pub fn part1() -> Cost {
    let grid = get_data();
    let goal = get_goal(&grid);
    let start = Complex::new(0, 0);

    lowest_cost(grid, start, goal)
}
pub fn part2() -> Cost {
    0
}

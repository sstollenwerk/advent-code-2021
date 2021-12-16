use crate::lib::to_filename;

use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

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

fn dijkastra(g: Grid) -> Cost {
    let mut unvisited: HashSet<Place> = g.keys().map(|c| *c).collect();

    let mut to_visit: HashSet<Place> = HashSet::new();

    let start = Complex::new(0, 0);
    let end = get_goal(&g);

    let mut costs: HashMap<Place, Cost> = HashMap::new();
    costs.insert(start, 0);

    let dirs = vec![
        Complex::new(0, 1),
        Complex::new(1, 0),
        Complex::new(0, -1),
        Complex::new(-1, 0),
    ];

    let mut node = start;

    loop {
        let cost = costs[&node];
        let nexts = dirs
            .iter()
            .map(|c| *c + node)
            .filter(|c| g.contains_key(&c)).filter(|c| unvisited.contains(&c) );

        for n in nexts {
            to_visit.insert(n);
            let current = costs.entry(n).or_insert(Cost::MAX);
            *current = *current.min(&mut ( cost + g[&n] ));
        }
        if node == end {
            break;
        }

        unvisited.remove(&node);
        to_visit.remove(&node);



        if let Some(n_) = to_visit
            .iter()
            .min_by_key(|c| costs[c])
        {
            node = *n_;
        } else {
            break;
        }
    }
   let res =  costs[&end];
   display(costs);
   res
}

fn lowest_cost(g: Grid) -> Cost {
    let mut places: Vec<Place> = g.keys().map(|c| *c).collect();
    places.sort_by_key(|c| (c.re, c.im));
    places.reverse();
    let start = Complex::new(0, 0);
    let end = places[0];

    let mut costs: HashMap<Place, Cost> = HashMap::new();
    costs.insert(end, 0);

    let dirs = vec![Complex::new(0, 1), Complex::new(1, 0)];

    for p in places.into_iter() {
        if costs.contains_key(&p) {
            continue;
        }
        let nexts = dirs.iter().map(|c| *c + p).filter(|c| g.contains_key(&c));

        let res = nexts.map(|c| g[&c] + costs[&c]).min().unwrap();

        costs.insert(p, res);
    }
    costs[&start]
}
fn extend(g: &Grid, size: u32) -> Grid {
    let mut dupes = Vec::new();

    for i in (0..size) {
        for j in (0..size) {
            dupes.push((i as u32, j as u32))
        }
    }
    let mut res = HashMap::new();

    let bottom = get_goal(&g);
    let (a, b) = (bottom.re + 1, bottom.im + 1);

    for (k, v) in g.iter() {
        for delta in dupes.iter() {
            let s = ((*v + (delta.0 + delta.1) - 1) % 9) + 1;
            let d = Complex::new(a * (delta.0 as i32), b * (delta.1 as i32));
            let pos = *k + d;
            res.insert(pos, s);
            //    println!("{:?}",(k,v,pos, s));
        }
        // panic!()
    }

    

    res
}

fn display(grid:Grid) -> () {
    let vals = grid
        .keys()
        .map(|c| (c.re as usize, c.im as usize))
        .collect::<Vec<_>>();
    let y_ = vals.iter().map(|t| t.0).max().unwrap();
    let x_ = vals.iter().map(|t| t.1).max().unwrap();

    let mut row = Vec::new();
    for _ in (0..=y_) {
        row.push(0)
    }

    let mut rows = Vec::new();
    for _ in (0..=x_) {
        rows.push(row.clone())
    }
    for (a, b) in vals.into_iter() {
        rows[b][a] = grid[&Complex::new(a as i32,b as i32)]
    }

    for r in rows.iter() {
        println!("{:?}", r);
    }
}

pub fn part1() -> Cost {
    let grid = get_data();

    let goal = get_goal(&grid);
    let start = Complex::new(0, 0);


   // lowest_cost(grid)
   dijkastra(grid)
}
pub fn part2() -> Cost {
    let grid = extend(&get_data(), 5);
    let goal = get_goal(&grid);

    let start = Complex::new(0, 0);
    dijkastra(grid)

}

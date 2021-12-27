use crate::lib::to_filename;
use crate::lib::transpose;

use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use cached::proc_macro::cached;
use num_complex::Complex;

type Num = u64;

type Place = Complex<Num>;
type Delta = Complex<Num>;
type Size = Complex<Num>;

type DirCucumbers = HashSet<Place>;

type Cucumbers = HashMap<Delta, DirCucumbers>;
type Grid = (Size, Cucumbers);

fn get_data() -> Grid {
    let mut grid = Cucumbers::new();

    let rows: Vec<_> = fs::read_to_string(to_filename(25))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect();

    println!("{:?}", rows[0]);

    let size = Size::new(rows[0].len() as Num, rows.len() as Num);

    for (i, row) in rows.iter().enumerate() {
        for (j, c_) in row.iter().enumerate() {
            let pos = Complex::new(j as Num, i as Num);

            if let Some(c) = *c_ {
                let vals = grid.entry(c).or_insert(HashSet::new());
                vals.insert(pos);
            }
        }
    }

    (size, grid)
}

fn read_row(row: &str) -> Vec<Option<Delta>> {
    row.chars()
        .map(|d| match d {
            '>' => Some(Delta::new(1, 0)),
            'v' => Some(Delta::new(0, 1)),
            '.' => None,
            _ => panic!(),
        })
        .collect::<Vec<_>>()
}

fn reposition(delta: Delta, cucumber: Place, size: Size) -> Place {
    let p = cucumber + delta;
    Place::new(p.re % size.re, p.im % size.im)
}

fn part_step(dir: Delta, g: Grid) -> Grid {
    let (size, grid_) = g;
    let current: HashSet<Place> = grid_.values().flatten().map(|c| *c).collect();
    let check = grid_[&dir].clone();

    let mut grid = grid_;
    let (stay, move_): (DirCucumbers, DirCucumbers) = check
        .into_iter()
        .partition(|c| current.contains(&reposition(dir, *c, size)));
    let move_: DirCucumbers = move_
        .into_iter()
        .map(|c| reposition(dir, c, size))
        .collect();
    let new_: DirCucumbers = stay.union(&move_).map(|c| *c).collect();

    grid.insert(dir, new_);
    (size, grid)
}

fn step(g_: &Grid) -> Grid {
    let mut g = g_.clone();

    for d in read_row(">v").into_iter() {
        g = part_step(d.unwrap(), g);
    }
    g
}

fn display(grid: &Grid) -> () {
    let (size, grid_) = grid;

    let row = vec!['.'; size.re.try_into().unwrap()];
    let mut rows = vec![row; size.im.try_into().unwrap()];

    let dirs_ = ">v";

    //    println!("{:?}", grid);

    // todo!();

    for (cha, dir) in dirs_.chars().zip(read_row(dirs_)) {
        let poses = &grid_[&dir.unwrap()];
        for c in poses {
            rows[c.im as usize][c.re as usize] = cha;
        }
    }

    //   rows = transpose(&rows);

    for r in rows.iter() {
        println!("{:?}", r.into_iter().collect::<String>());
    }

    println!("")
}

pub fn part1() -> Num {
    let mut g = get_data();

    //  display(&g);

    for i in (1..) {
        let res = step(&g);
        if res == g {
            display(&g);
            return i;
        } else {
            g = res;
            // display(&g)
        }
        if (i % 100 == 0) {
            println!("{:?}", i);
        }
    }
    panic!();
}

pub fn part2() -> Num {
    todo!();
}

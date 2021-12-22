use crate::lib::to_filename;
use crate::lib::transpose;

use std::fs;

//use std::collections::HashMap;
use std::collections::HashSet;

use itertools::{iproduct, Itertools};

type Num = i64;

type Position = Num;

type Pair = (Position, Position);
type CubeRange = (Pair, Pair, Pair);
type Cube = (Position, Position, Position);

#[derive(Debug, Clone, std::cmp::PartialEq, Copy)]
enum State {
    On,
    Off,
}

type Row = (State, CubeRange);

fn get_data() -> Vec<Row> {
    fs::read_to_string(to_filename(22))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect()
}

fn read_row(row: &str) -> Row {
    let parts: Vec<&str> = row.split(' ').collect();
    assert_eq!(parts.len(), 2);
    let s = match parts[0] {
        "on" => State::On,
        "off" => State::Off,
        _ => panic!(),
    };

    let cubes = parts[1].split(',');
    let cubes = cubes.map(|s| s.split('=').nth(1).unwrap().split("..").collect::<Vec<_>>());
    let cubes: Vec<Pair> = cubes
        .map(|p| (p[0].parse::<Num>().unwrap(), p[1].parse::<Num>().unwrap()))
        .collect();
    let cubes = (cubes[0], cubes[1], cubes[2]);
    (s, cubes)
}

fn keep_(p: Pair) -> bool {
    vec![p.0, p.1].iter().map(|n| n.abs()).max().unwrap() <= 50
}

fn keep(r: Row) -> bool {
    let p = r.1;
    vec![p.0, p.1, p.2].iter().all(|&n| keep_(n))
}

fn all_cubes(c: CubeRange) -> HashSet<Cube> {
    let mut res: HashSet<Cube> = HashSet::new();
    let parts: Vec<_> = vec![c.0, c.1, c.2]
        .into_iter()
        .map(|p| (p.0..=p.1))
        .collect();
    for s in iproduct!(parts[0].clone(), parts[1].clone(), parts[2].clone()) {
        res.insert(s);
    }
    res
}

fn initialization_brute_force(rows: &Vec<Row>) -> Num {
    let mut res: HashSet<Cube> = HashSet::new();

    for r in rows.iter() {
        let (state, row) = r;
        let cubes = all_cubes(*row);
        res = match state {
            State::On => res.union(&cubes).map(|x| *x).collect(),
            State::Off => res.difference(&cubes).map(|x| *x).collect(),
        };
    }
    res.len().try_into().unwrap()
}

pub fn part1() -> Num {
    let mut vals = get_data();
    vals.retain(|p| keep(*p));

    initialization_brute_force(&vals)
   // todo!();
}
pub fn part2() -> Num {
    // let vals = get_data();
    todo!();
}

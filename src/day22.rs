use crate::lib::to_filename;
use crate::lib::transpose;

use std::cmp;
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

fn intersection(a: CubeRange, b: CubeRange) -> Option<CubeRange> {
    let a = [a.0, a.1, a.2];
    let b = [b.0, b.1, b.2];
    let mut intersections = Vec::new();
    for p in a.into_iter().zip(b.into_iter()) {
        let (a0, a1) = p.0;
        assert!(a0 <= a1);
        let (b0, b1) = p.1;
        assert!(b0 <= b1);

        let (c, d) = (cmp::max(a0, b0), cmp::min(a1, b1));
        if c > d {
            return None;
        } else {
            intersections.push((c, d));
        }
    }

    Some((intersections[0], intersections[1], intersections[2]))
}

fn initialization_efficient(rows: &Vec<Row>) -> Num {
    let mut cuboids: Vec<CubeRange> = Vec::new();
    let mut intersections: Vec<CubeRange> = Vec::new();
    for r in rows.iter() {
        let (state, row) = r;
        let mut new_cubes = Vec::new();
        let mut new_intersections = Vec::new();
        for c in cuboids.iter() {
            if let Some(k) = intersection(*row, *c) {
                new_intersections.push(k)
            }
        }

        for c in intersections.iter() {
            if let Some(k) = intersection(*row, *c) {
                new_cubes.push(k)
            }
        }
        match state {
            State::On => cuboids.push(*row),
            State::Off => (),
        };

        cuboids.append(&mut new_cubes);
        intersections.append(&mut new_intersections);

     //   println!("{:?}", r);
     //   println!("{:?}", cuboids);
    //    println!("{:?}", intersections);

    //    println!("");
    }

    cuboids.into_iter().map(|c| size(c)).sum::<Num>()
        - intersections.into_iter().map(|c| size(c)).sum::<Num>()
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

fn size(c: CubeRange) -> Num {
    vec![c.0, c.1, c.2]
        .into_iter()
        .map(|p| (p.1 - p.0 + 1))
        .product()
}

pub fn part1() -> Num {
    let mut vals = get_data();

    vals.retain(|p| keep(*p));

  //  let vals = vec![vals[10], vals[0], vals[10], vals[0], vals[10], vals[10], vals[10], vals[1], vals[3] ];
    //println!("{:?}", vals);


    assert_eq!(initialization_efficient(&vals),initialization_brute_force(&vals)  );

    initialization_efficient(&vals)
    // todo!();
}
pub fn part2() -> Num {
    let  vals = get_data();

    initialization_efficient(&vals)
}

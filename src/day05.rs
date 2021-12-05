use crate::lib::to_filename;
use core::ops::Range;
use std::fs;

use counter::Counter;

type Place = (i32, i32);
type Pair = (Place, Place);

fn get_data() -> Vec<Pair> {
    fs::read_to_string(to_filename(5))
        .expect("Could not read file")
        .lines()
        .map(read_row)
        .collect()
}

fn read_row(row: &str) -> Pair {
    let data = row
        .replace(" -> ", ",")
        .replace(" ", "")
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(data.len(), 4);

    ((data[0], data[1]), (data[2], data[3]))
}

fn horiz(p: &Pair) -> bool {
    p.0 .0 == p.1 .0
}

fn vert(p: &Pair) -> bool {
    p.0 .1 == p.1 .1
}

fn diag(p: &Pair) -> bool {
    !(horiz(p) || vert(p))
}

fn ex_range(a: i32, b: i32) -> Range<i32> {
    if a > b {
        ex_range(b, a)
    } else {
        (a..(b + 1))
    }
}

fn places(p: &Pair) -> Vec<Place> {
    if horiz(p) {
        let (a, b) = (p.0 .1, p.1 .1);
        ex_range(a, b).map(|x| (p.0 .0, x)).collect()
    } else if vert(p) {
        let (a, b) = (p.0 .0, p.1 .0);
        ex_range(a, b).map(|x| (x, p.0 .1)).collect()
    } else {
        panic!()
    }
}

pub fn part1() -> i32 {
    let mut lines = get_data();

    lines.retain(|x| !diag(x));

    println!("{:?}", lines);

    let places = lines.iter().map(|p| places(p)).flatten();

    places
        .collect::<Counter<_>>()
        .values()
        .map(|x| *x as i32)
        .filter(|x| x > &1)
        .count() as i32
}

pub fn part2() -> i32 {
    let mut lines = get_data();

    //  lines.retain( |x| !diag(x) );

    println!("{:?}", lines);

    let places = lines.iter().map(|p| places(p)).flatten();

    places
        .collect::<Counter<_>>()
        .values()
        .map(|x| *x as i32)
        .filter(|x| x > &1)
        .count() as i32
}

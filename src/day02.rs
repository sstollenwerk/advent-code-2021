use crate::lib::to_filename;

use std::fs;

use std::collections::HashMap;

use num_complex::Complex;

fn get_data() -> Vec<Complex<i32>> {
    fs::read_to_string(to_filename(2))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect()
}

fn read_row(row: &str) -> Complex<i32> {
    let directions = HashMap::from([
        ("forward", Complex::new(1, 0)),
        ("down", Complex::new(0, 1)),
        ("up", Complex::new(0, -1)),
    ]);
    let data = row.split_whitespace().collect::<Vec<_>>();

    assert_eq!(data.len(), 2);

    let dir = directions.get(&data[0]).unwrap();

    let dist = (data[1]).parse::<i32>().unwrap();

    dir.scale(dist)
}
pub fn part1() -> i32 {
    let rows = get_data();

    let sum: Complex<i32> = rows.iter().sum();

    sum.re * sum.im
}

pub fn part2() -> i32 {
    let mut aim = 0;
    let mut position = Complex::new(0, 0);

    for c in get_data().into_iter() {
        aim += c.im;
        // if forward, im is 0, += 0 does nothing
        position += Complex::new(c.re, c.re * aim);
        // if not forward, re is 0, += 0 does nothing
        // in either case, no check is needed to see what direction it is.
    }
    let res = position;
    res.re * res.im
}

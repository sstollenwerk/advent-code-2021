use crate::lib::to_filename;

use std::fs;

fn get_data() -> Vec<Vec<u32>> {
    fs::read_to_string(to_filename(3))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect()
}

fn read_row(row: &str) -> Vec<u32> {
    row.chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
}

fn transpose<T: Copy + std::fmt::Debug>(vals: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut res = Vec::new();

    for k in (0..(vals[0].len())) {
        let mut row = Vec::new();

        for r in vals.iter() {
            row.push(r[k]);
        }
        res.push(row);
    }
    res
}

fn gamma(trans: &Vec<Vec<u32>>) -> i32 {
    let amounts = trans
        .iter()
        .map(|r| (((r.iter().sum::<u32>() * 2) > r.len().try_into().unwrap()) as u32));

    let as_bin: String = amounts.map(|c| char::from_digit(c, 10).unwrap()).collect();

    i32::from_str_radix(&as_bin, 2).unwrap()
}

fn epsilon(trans: &Vec<Vec<u32>>) -> i32 {
    let amounts = trans
        .iter()
        .map(|r| (((r.iter().sum::<u32>() * 2) < r.len().try_into().unwrap()) as u32));

    let as_bin: String = amounts.map(|c| char::from_digit(c, 10).unwrap()).collect();

    i32::from_str_radix(&as_bin, 2).unwrap()
}

pub fn part1() -> i32 {
    let data = get_data();

    let trans = transpose(data);

    let gamma = gamma(&trans);
    let epsilon = epsilon(&trans); // tried bitwise not, didn't work

    println!("{:?}", gamma);
    println!("{:?}", epsilon);

    (gamma * epsilon)
}

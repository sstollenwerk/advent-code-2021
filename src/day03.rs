use crate::lib::to_filename;

use std::fs;

fn get_data() -> Vec<Vec<u32>> {
    fs::read_to_string(to_filename(3))
        .expect("Could not read file")
        .lines()
        .map(read_row)
        .collect()
}

fn read_row(row: &str) -> Vec<u32> {
    row.chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
}

fn transpose<T: Copy + std::fmt::Debug>(vals: &Vec<Vec<T>>) -> Vec<Vec<T>> {
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

fn gamma(trans: &Vec<Vec<u32>>) -> Vec<u32> {
    trans.iter().map(|r| (one_common(r) as u32)).collect()
}

fn epsilon(trans: &Vec<Vec<u32>>) -> Vec<u32> {
    trans.iter().map(|r| (!one_common(r) as u32)).collect()
}

fn one_common(r: &Vec<u32>) -> bool {
    ((r.iter().sum::<u32>() * 2) >= r.len().try_into().unwrap())
}

fn as_num(r: Vec<u32>) -> i32 {
    let as_bin: String = r
        .iter()
        .map(|c| char::from_digit(*c, 10).unwrap())
        .collect();

    i32::from_str_radix(&as_bin, 2).unwrap()
}

pub fn part1() -> i32 {
    let data = get_data();

    let trans = transpose(&data);

    let gamma = as_num(gamma(&trans));
    let epsilon = as_num(epsilon(&trans)); // tried bitwise not, didn't work

    (gamma * epsilon)
}

fn rating(data: &Vec<Vec<u32>>, want_more: bool) -> Vec<u32> {
    let mut data: Vec<Vec<u32>> = data.clone();

    let mut i = 0;

    while (data).len() > 1 {
        let mut section = Vec::new();
        for r in data.iter() {
            section.push(r[i]);
        }

        let keep = (one_common(&section) == want_more) as u32;

        data.retain(|x| x[i] == keep);

        i += 1
    }
    (*data[0]).to_vec()
}

pub fn part2() -> i32 {
    let data = get_data();

    let oxygen_gen = rating(&data, true);
    let c02_scrub = rating(&data, false);

    as_num(oxygen_gen) * as_num(c02_scrub)
}

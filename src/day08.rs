use crate::lib::to_filename;
use std::fs;

use std::collections::HashSet;

type Digit = HashSet<char>;

type Digits = Vec<Digit>;

fn get_data() -> Vec<(Digits, Digits)> {
    fs::read_to_string(to_filename(8))
        .expect("Could not read file")
        .lines()
        .map(read_row)
        .collect()
}

fn read_row(row: &str) -> (Digits, Digits) {
    let data = row.split("|").map(read_part).collect::<Vec<_>>();

    assert_eq!(data.len(), 2);
    (data[0].clone(), data[1].clone())
}

fn read_part(part: &str) -> Digits {
    part.split(" ")
        .filter(|s| s.len() > 0)
        .map(|s| s.chars().collect())
        .collect()
}

pub fn part1() -> i32 {
    let posses: HashSet<_> = [2, 4, 3, 7].iter().cloned().collect();

    let parts = get_data();
    let outputs = parts.into_iter().map(|x| x.1).flatten();
    let uniques = outputs.filter(|x| posses.contains(&(x.len())));
    uniques.count().try_into().unwrap()
}

pub fn part2() -> i32 {
    0
}

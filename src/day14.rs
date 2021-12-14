use crate::lib::to_filename;

use std::fs;

use std::collections::HashMap;

use counter::Counter;

use itertools::{Either, Itertools};

type Line = String;

type Inserts = HashMap<Line, Line>;

type Res = u64;

fn get_data() -> (Line, Inserts) {
    let (lines, changes_): (Vec<_>, Vec<_>) = fs::read_to_string(to_filename(14))
        .expect("Could not read file")
        .lines()
        .filter(|s| s.len() > 0)
        .map(|s| read_row(s))
        .partition_map(|r| r);

    let line: Line = lines[0].clone();
    let changes: Inserts = changes_.into_iter().collect();
    (line, changes)
}

fn read_row(row: &str) -> Either<Line, (Line, Line)> {
    if row.contains(" -> ") {
        let data = row.split(" -> ").collect::<Vec<_>>();
        return itertools::Either::Right((data[0].to_string(), data[1].to_string()));
    } else {
        return itertools::Either::Left(row.to_string());
    }
}

fn pairwise(pairs: Counter<Line>, changes: &Inserts) -> Counter<Line> {
    let mut amts: Counter<String> = Counter::new();
    for (k_, v) in pairs.iter() {
        let k: Vec<char> = k_.chars().collect();
        let a = k[0];
        let c = k[1];
        let b = (changes.get(k_).unwrap()).chars().next().unwrap();
        let s: String = vec![a, b].iter().collect();
        let s2: String = vec![b, c].iter().collect();

        amts[&s] += *v;
        amts[&s2] += *v;
    }
    amts
}

fn amounts(line: Line, steps: u32, changes: &Inserts) -> Counter<char> {
    let size = 2;
    let k = line.len();

    let amts_start: Counter<String> = (0..=k - size)
        .map(|i| (&line[i..i + size]).to_string())
        .collect();

    let line_: Vec<char> = line.chars().collect();

    let mut amts = amts_start;
    for _ in (0..steps) {
        amts = pairwise(amts, changes);
    }

    let mut res = Counter::new();

    for (k, v) in amts.iter() {
        let c = k.chars().next().unwrap();
        res[&c] += v;
    }

    res[line_.last().unwrap()] += 1;
    res
}

fn step(line: Line, changes: &Inserts) -> Line {
    let size = 2;
    let k = line.len();

    let mut inserts = (0..=k - size).map(|i| changes.get(&line[i..i + size]));

    let mut chars_ = line.chars();

    let mut res = String::new();

    res.push(chars_.next().unwrap());

    for j in (1..k) {
        if let Some(Some(i)) = inserts.next() {
            res.push_str(i);
        } else {
            println!("{:?}", j);
        }
        res.push(chars_.next().unwrap());
    }

    res
}

pub fn part1() -> Res {
    let (line_, changes) = get_data();
    let mut line = line_;
    for _ in (0..10) {
        line = step(line, &changes);
    }

    let amts: Counter<char> = line.chars().collect();
    (amts.values().max().unwrap() - amts.values().min().unwrap()) as Res
}
pub fn part2() -> Res {
    let (line, changes) = get_data();

    let amts: Counter<char> = amounts(line, 40, &changes);
    (amts.values().max().unwrap() - amts.values().min().unwrap()) as Res
}

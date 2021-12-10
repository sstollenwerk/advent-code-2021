use crate::lib::to_filename;

use std::fs;

use std::collections::HashMap;

type Closed = bool;
type Score = i32;

type Value = (char, Closed, Score);
type Row = Vec<Value>;

fn get_data() -> Vec<Row> {
    fs::read_to_string(to_filename(10))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect()
}

fn read_row(row: &str) -> Row {
    row.chars().map(|c| read_char(&c)).collect::<Vec<_>>()
}

fn read_char(c: &char) -> Value {
    let convs = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let scores = HashMap::from([('(', 3), ('[', 57), ('{', 1197), ('<', 25137)]);
    let closed = convs.contains_key(c);
    let c_ = *convs.get(&c).unwrap_or(c);
    let score = *scores.get(&c_).unwrap();
    (c_, closed, score)
}

fn parse(row: Row) -> Result<Row, Value> {
    let mut stack = Vec::new();
    for v in row.into_iter() {
        if !v.1 {
            stack.push(v)
        } else {
            let prev_ = stack.pop().unwrap();
            if prev_.0 != v.0 {
                return Err(v);
            }
        }
    }
    return Ok(stack);
}

pub fn part1() -> Score {
    let rows = get_data();
    let errs = rows
        .iter()
        .map(|r|  parse(r.to_vec()) )
        .filter(|r| r.is_err())
        .map(|x| x.unwrap_err());
    errs.map(|r| r.2).sum()
}

pub fn part2() -> Score {
    0
}

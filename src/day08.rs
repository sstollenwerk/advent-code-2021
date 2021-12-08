use crate::lib::to_filename;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

type Digit = Vec<char>;
type Digits = Vec<Digit>;

type Row = (Digits, Digits);

fn get_data() -> Vec<Row> {
    fs::read_to_string(to_filename(8))
        .expect("Could not read file")
        .lines()
        .map(read_row)
        .collect()
}

fn read_row(row: &str) -> Row {
    let data = row.split("|").map(read_part).collect::<Vec<_>>();

    assert_eq!(data.len(), 2);
    (data[0].clone(), data[1].clone())
}

fn read_part(part: &str) -> Digits {
    part.split(" ")
        .filter(|s| s.len() > 0)
        .map(s_conv)
        .collect()
}

fn s_conv(s: &str) -> Digit {
    s.chars().sorted().collect::<Digit>()
}
fn v_conv(mut v: Digit) -> Digit {
    v.sort();
    v
}

fn translate<T: std::hash::Hash + std::cmp::Eq, V: std::hash::Hash + std::cmp::Eq + Copy>(
    translator: &HashMap<T, V>,
    data: &Vec<T>,
) -> Vec<V> {
    data.iter().map(|x| *translator.get(&x).unwrap()).collect()
}

fn to_num(d: Vec<i32>) -> i32 {
    let mut ans = 0;
    for n in d.iter() {
        ans *= 10;
        ans += n;
    }
    ans
}

fn solve(parts: Row) -> i32 {
    let sections_ = HashMap::from([
        (0, "abcefg"),
        (1, "cf"),
        (2, "acdeg"),
        (3, "acdfg"),
        (4, "bcdf"),
        (5, "abdfg"),
        (6, "abdefg"),
        (7, "acf"),
        (8, "abcdefg"),
        (9, "abcdgf"),
    ]);

    let mut sections = HashMap::new();
    for (k, val) in sections_.iter() {
        sections.insert(s_conv(val), k);
    }

    let digits = "abcdefg";
    // 5040 permutations, not too many to brute force I think

    let (all_digits, check) = parts;

    let mut answers = Vec::new();

    for p in digits.chars().permutations(digits.len()) {
        let poss = digits.chars().zip(p).collect::<HashMap<char, char>>();

        let res = all_digits
            .iter()
            .map(|d| v_conv(translate(&poss, d)))
            .collect::<Vec<_>>();

        let reference = (res.iter().filter_map(|p| sections.get(p)))
            .sorted()
            .collect::<Vec<_>>();

        if reference == sections.values().sorted().collect::<Vec<_>>() {
            let p = check
                .iter()
                .map(|d| **sections.get(&v_conv(translate(&poss, d))).unwrap())
                .collect::<Vec<i32>>();
            println!("{:?}", p);

            answers.push(to_num(p));
        }
    }
    println!("{:?}", answers);

    answers[0]

    // .chars().collect()
}

pub fn part1() -> i32 {
    let posses: HashSet<_> = [2, 4, 3, 7].iter().cloned().collect();

    let parts = get_data();
    let outputs = parts.into_iter().map(|x| x.1).flatten();
    let uniques = outputs.filter(|x| posses.contains(&(x.len())));
    uniques.count().try_into().unwrap()
}

pub fn part2() -> i32 {
    get_data().into_iter().map(|x| solve(x)).sum()
}

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

fn group_by<T: std::clone::Clone + std::ops::Deref, V: std::hash::Hash + std::cmp::Eq + Copy>(
    data: &Vec<T>,
    key: impl Fn(&T) -> V,
) -> HashMap<V, Vec<T>> {
    let mut groups = HashMap::new();
    for d in data.iter() {
        let k = key(d);
        let vals = groups.entry(k).or_insert(Vec::new());
        // *vals.push(*d);
        vals.push(d.clone());
        let v = vals.to_vec();

        groups.insert(k, v);
    }

    groups
}

fn flip<T: Copy, V: std::hash::Hash + std::cmp::Eq + Copy>(data: HashMap<T, V>) -> HashMap<V, T> {
    let mut res = HashMap::new();

    for (k, v) in data.iter() {
        res.insert(*v, *k);
    }

    res
}

fn non_copy_partition<T: Copy>(
    vals: Vec<HashSet<T>>,
    cond: impl Fn(&HashSet<T>) -> bool,
) -> (Vec<HashSet<T>>, Vec<HashSet<T>>) {
    let mut t = Vec::new();
    let mut f = Vec::new();
    for v in vals.iter() {
        if cond(&v) {
            t.push(v.clone())
        } else {
            f.push(v.clone())
        }
    }

    (t, f)
}

fn solve(parts: Row) -> i32 {
    let (all_digits, check) = parts;
    let all_digits: Digits = all_digits.into_iter().map(v_conv).collect();
    let sizes_ = group_by(&all_digits, |x| x.len());
    let mut sizes = HashMap::new();
    for (k, v) in sizes_.iter() {
        sizes.insert(
            k,
            v.iter()
                .map(|s| s.iter().collect::<HashSet<_>>())
                .collect::<Vec<_>>(),
        );
    }

    let mut knowns = HashMap::new();

    knowns.insert(1, &sizes.get(&2).unwrap()[0]);
    knowns.insert(7, &sizes.get(&3).unwrap()[0]);
    knowns.insert(4, &sizes.get(&4).unwrap()[0]);
    knowns.insert(8, &sizes.get(&7).unwrap()[0]);

    let (others, sixes): (Vec<HashSet<_>>, Vec<HashSet<_>>) =
        non_copy_partition(sizes.get(&6).unwrap().to_vec(), |s| {
            s.is_superset(knowns.get(&1).unwrap())
        });

    knowns.insert(6, sixes.iter().next().unwrap());

    let (nines, zeros): (Vec<HashSet<_>>, Vec<HashSet<_>>) =
        non_copy_partition(others.to_vec(), |s| s.is_superset(knowns.get(&4).unwrap()));

    knowns.insert(9, nines.iter().next().unwrap());
    knowns.insert(0, zeros.iter().next().unwrap());

    let (others, sixes): (Vec<HashSet<_>>, Vec<HashSet<_>>) =
        non_copy_partition(sizes.get(&6).unwrap().to_vec(), |s| {
            s.is_superset(knowns.get(&1).unwrap())
        });

    let (fives, others): (Vec<HashSet<_>>, Vec<HashSet<_>>) =
        non_copy_partition(sizes.get(&5).unwrap().to_vec(), |s| {
            s.is_subset(knowns.get(&6).unwrap())
        });

    knowns.insert(5, fives.iter().next().unwrap());

    let (threes, twos): (Vec<HashSet<_>>, Vec<HashSet<_>>) =
        non_copy_partition(others.to_vec(), |s| s.is_superset(knowns.get(&1).unwrap()));

    knowns.insert(3, threes.iter().next().unwrap());
    knowns.insert(2, twos.iter().next().unwrap());

    let mut map_: HashMap<Vec<char>, i32> = HashMap::new();
    for (k, v) in knowns.iter() {
        map_.insert(v_conv(Vec::from_iter(v.iter().map(|x| **x))), *k);
    }

    to_num(
        check
            .iter()
            .map(|x| *map_.get(&(x.to_vec())).unwrap())
            .collect::<Vec<i32>>(),
    )

    /*
    know
    1,4,7,8


    items with 6 elements are 0,6,9
    1 subset of 0,9 . not 6
    know 6
    4 subset 9, not 0. can distingush

    know 0,1,4,6,7,8,9


    remaining are 2,3,5, all with 5 elements.
    only 5 is a subset of 6, know 5

    2,3 remain.
    3 is superset of 1, 2 isn't
    */
}

fn solve_ineff(parts: Row) -> i32 {
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

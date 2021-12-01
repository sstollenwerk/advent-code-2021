use crate::lib::read_day;

use itertools::Itertools;

pub fn part1() -> usize {
    let data = read_day(1);

    largers(data)
}

pub fn part2() -> usize {
    let data = read_day(1);

    let mut sizes: Vec<i32> = Vec::new();
    for (a, b, c) in data.into_iter().tuple_windows() {
        sizes.push((a + b + c));
    }

    largers(sizes)
}

fn largers(data: Vec<i32>) -> usize {
    let mut pairs: Vec<(i32, i32)> = Vec::new();
    for (a, b) in data.into_iter().tuple_windows() {
        pairs.push((a, b));
    }

    pairs.iter().filter(|x| x.0 < x.1).collect::<Vec<_>>().len()
}

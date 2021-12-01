use crate::lib::read_day;

use itertools::Itertools;

pub fn part1() -> usize {
    let data = read_day(1);

    let mut pairs: Vec<(i32, i32)> = Vec::new();
    for (a, b) in data.into_iter().tuple_windows() {
        pairs.push((a, b));
    }

    pairs.iter().filter(|x| x.0 < x.1).collect::<Vec<_>>().len()
}

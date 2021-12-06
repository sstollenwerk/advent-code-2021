use crate::lib::to_filename;
//use core::ops::Range;
use std::fs;

use counter::Counter;

fn get_data() -> Vec<i32> {
    read_row(
        fs::read_to_string(to_filename(6))
            .expect("Could not read file")
            .lines()
            .next()
            .unwrap(),
    )
}

fn read_row(row: &str) -> Vec<i32> {
    row.split(",").map(|s| s.parse::<i32>().unwrap()).collect()

    // returning vec instead of counter because unsure about part 2
}

fn day(amts: &Counter<i32>) -> Counter<i32> {
    let mut res = Counter::new();

    for (day, amt) in amts.iter() {
        if day >= &1 {
            res[&(day - 1)] += amt;
        } else {
            res[&(6)] += amt;
            res[&(8)] += amt;
        }
    }
    //  println!("{:?}",res);

    res
}

pub fn part1() -> i32 {
    let mut remainings: Counter<i32> = get_data().iter().map(|x| *x as i32).collect();
    //   println!("{:?}",remainings);

    for _ in (0..80) {
        remainings = day(&remainings);
    }

    remainings.values().map(|x| *x as i32).sum()
}

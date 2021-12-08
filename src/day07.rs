use crate::lib::to_filename;
use std::fs;

fn get_data() -> Vec<i32> {
    read_row(
        fs::read_to_string(to_filename(7))
            .expect("Could not read file")
            .lines()
            .next()
            .unwrap(),
    )
}

fn read_row(row: &str) -> Vec<i32> {
    row.split(",").map(|s| s.parse::<i32>().unwrap()).collect()
}

fn dist_old(places: &Vec<i32>, n: i32) -> i32 {
    places.iter().map(|x| (*x - n).abs()).sum()
}

pub fn part1() -> i32 {
    let places = get_data();
    //   let places = vec![16,1,2,0,4,2,7,1,2,14];
    // tried finding mean, turned out to not be the correct way to solve the problem.

    let r = (places.iter().min().unwrap() + 0..(places.iter().max().unwrap()) + 1);
    let m = r.min_by_key(|x| dist_old(&places, *x)).unwrap();

    places.iter().map(|x| (*x - m).abs()).sum()
}

fn tri(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

fn dist(places: &Vec<i32>, n: i32) -> i32 {
    places.iter().map(|x| tri((*x - n).abs())).sum()
}

pub fn part2() -> i32 {
    let places = get_data();
    //   let places = vec![16,1,2,0,4,2,7,1,2,14];

    let r = (places.iter().min().unwrap() + 0..(places.iter().max().unwrap()) + 1);
    let m = r.min_by_key(|x| dist(&places, *x)).unwrap();

    dist(&places, m)
}

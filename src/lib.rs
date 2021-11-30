use std::fs;

pub fn read_nums(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename)
        .expect("Could not read file")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    contents
}

pub fn read_day(day: i32) -> Vec<i32> {
    let filename = format!("input/{}", day);
    read_nums(&filename)
}
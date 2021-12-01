use std::fs;

pub fn read_nums(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .expect("Could not read file")
        .lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

pub fn read_day(day: i32) -> Vec<i32> {
    let filename = format!("input/{:0>2}.txt", day);
    read_nums(&filename)
}

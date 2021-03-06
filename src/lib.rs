use std::fs;

pub fn read_nums(filename: &str) -> Vec<i32> {
    fs::read_to_string(filename)
        .expect("Could not read file")
        .lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

pub fn read_day(day: i32) -> Vec<i32> {
    let filename = to_filename(day);
    read_nums(&filename)
}

pub fn to_filename(day: i32) -> String {
    format!("input/{:0>2}.txt", day)
}

pub fn transpose<T: Copy + std::fmt::Debug>(vals: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut res = Vec::new();

    for k in (0..(vals[0].len())) {
        let mut row = Vec::new();

        for r in vals.iter() {
            row.push(r[k]);
        }
        res.push(row);
    }
    res
}

pub fn sorted<T: std::cmp::Ord + std::clone::Clone>(vals: &Vec<T>) -> Vec<T> {
    let mut v = vals.clone();
    v.sort();
    v.to_vec()
}

/*
fn as_base(data: &Vec<T>, base: T) -> T {
    //  println!("{:?}", ( data, base) );
    let mut res  = 0;
    for i in data {
        res *= base;
        res += *i;
    }
    res
}
*/
// doing funcs on generic ints is hard

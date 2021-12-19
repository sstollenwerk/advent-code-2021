use crate::lib::to_filename;

use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

use itertools::{iproduct, Itertools};

use std::ops::{Add, Sub};

type Num = i32;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
struct Point {
    x: Num,
    y: Num,
    z: Num,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point {
    fn new(x: Num, y: Num, z: Num) -> Point {
        Point { x, y, z }
    }

    fn abs(self) -> Point {
        Point {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    fn sorted(self) -> Point {
        let mut m = vec![self.x, self.y, self.z];
        m.sort();
        Point {
            x: m[0],
            y: m[1],
            z: m[2],
        }
    }

    fn manhatten(self, other: &Point) -> Num {
        let k = self - *other;
        let mut m = vec![k.x.abs(), k.y.abs(), k.z.abs()];
        m.iter().sum::<Num>()
    }
}

type Scanner = Vec<Point>;

fn get_data() -> Vec<Scanner> {
    fs::read_to_string(to_filename(19))
        .expect("Could not read file")
        .lines()
        .collect::<Vec<&str>>()
        .split(|r| r.contains("---"))
        .filter(|r| r.len() > 0)
        .map(|s| read_group(s.to_vec()))
        .collect()
}

fn read_group(rows: Vec<&str>) -> Scanner {
    rows.iter()
        .filter(|r| r.len() > 0)
        .map(|s| read_row(s))
        .collect()
}

fn read_row(row: &str) -> Point {
    let parts = row
        .split(',')
        .map(|n| n.parse::<Num>().unwrap())
        .collect::<Vec<_>>();
    Point::new(parts[0], parts[1], parts[2])
}

fn overlap(a: &Scanner, b: &Scanner) -> Option<(Scanner, Point)> {
    //println!("{:?}", a);
    //   println!("{:?}", b);
    let mut sames = Vec::new();
    for (i, p0) in a.iter().enumerate() {
        for (j, p1) in b.iter().enumerate() {
            let amt = check(p0, a, p1, b);
            if amt >= 12 {
                //            println!("{:?}", (i, p0, j, p1, amt));
                sames.push((p0, p1));
                // know that p0 == p1
            }
        }
    }
    //   println!("{:?}", sames);

    if sames.len() == 0 {
        return None;
    }

    let c: Vec<_> = sames.iter().map(|p| p.0).collect();
    let d: Vec<_> = sames.iter().map(|p| p.1).collect();

    //  println!("{:?}", rotations(d[0]).len() );

    for (i, p0) in rotations(d[0]).iter().enumerate() {
        let delta = *c[0] - *p0;
        println!("{:?}", delta);

        let poss_ = b
            .iter()
            .map(|p| rotations(p)[i] + delta)
            .collect::<HashSet<_>>();
        let a_ = a.iter().map(|p| *p).collect::<HashSet<Point>>();

        let both: HashSet<_> = a_.intersection(&poss_).collect::<HashSet<_>>();
        if both.len() >= 12 {
            return Some((a_.union(&poss_).map(|p| *p).collect::<Vec<_>>(), delta));
        }
    }
    panic!();
}

fn check(p0: &Point, a: &Scanner, p1: &Point, b: &Scanner) -> usize {
    let c = differences(*p0, a);
    let d = differences(*p1, b);
    //  let totals = Vec::new();
    let c_ = c
        .iter()
        .map(|p| p.abs())
        .map(|p| p.sorted())
        .collect::<HashSet<_>>();
    let d_ = d
        .iter()
        .map(|p| p.abs())
        .map(|p| p.sorted())
        .collect::<HashSet<_>>();

    let both: HashSet<_> = c_.intersection(&d_).collect::<HashSet<_>>();
    let amt = both.len();
    /*   println!("{:?}", c_.iter().sorted().collect::<Vec<_>>());
    println!("{:?}", d_.iter().sorted().collect::<Vec<_>>());
    println!("{:?}",amt);
    println!(
        "{:?}",
        both.contains(&Point {
            x: 81,
            y: 163,
            z: 1
        }.sorted())
    );
    println!("");
    */
    amt
}

fn rotations(p: &Point) -> Vec<Point> {
    let mut res = Vec::new();
    for m in vec![0 as usize, 1 as usize, 2 as usize]
        .iter()
        .permutations(3)
    {
        let (a, b, c) = (m[0], m[1], m[2]);
        let vals = [-1, 1];
        for n in iproduct!(vals.iter(), vals.iter(), vals.iter()) {
            let (i, j, k) = (n.0, n.1, n.2);

            let p = vec![p.x, p.y, p.z];

            res.push(Point::new(p[*a] * i, p[*b] * j, p[*c] * k));
        }
    }
    res
}

fn differences(p: Point, s: &Scanner) -> Scanner {
    s.iter().map(|a| *a - p).collect()
}

fn all_scanners(mut vals: Vec<Scanner>) -> (Scanner, Vec<Point>) {
    let mut current = vals.remove(0);
    let mut scans = vec![Point::new(0,0,0)];
    while vals.len() > 0 {
        for i in (0..vals.len()) {
            if let Some((k, scan)) = overlap(&current, &vals[i]) {
                current = k;
                println!("{:?}", i);
                println!("{:?}", vals.len());
                scans.push(scan);
                vals.remove(i);

                break;
            }
        }
    }

    (current, scans)
}

pub fn part1() -> Num {
    let vals = get_data();

    all_scanners(vals).0.len() as Num
}
pub fn part2() -> Num {
    let vals = get_data();

    let mut total = all_scanners(vals).1;
    total.sort_by_key(|p| (p.x, p.y, p.z));
    println!("{:?}", total);


    let mut dists = Vec::new();

    for (a, b) in iproduct!(total.iter(), total.iter()) {

        dists.push(a.manhatten(b));
    }
    *dists.iter().max().unwrap()
}

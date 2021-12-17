use crate::lib::to_filename;

use std::fs;

use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

use core::ops::RangeInclusive;

use num_complex::Complex;

type Num = i32;

type Position = Complex<Num>;
type NumRange = (Num, Num);
type Target = (NumRange, NumRange);

fn get_data() -> Target {
    fs::read_to_string(to_filename(17))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .next()
        .unwrap()
}

fn read_row(row: &str) -> Target {
    let mut parts = row.split('=');
    parts.next();

    let sections: Vec<NumRange> = parts
        .map(|p| make_range(p.split(", ").nth(0).unwrap()))
        .collect();

    (sections[0], sections[1])
}

fn make_range(row: &str) -> NumRange {
    let mut parts: Vec<Num> = row.split("..").map(|n| n.parse::<Num>().unwrap()).collect();
    parts.sort();
    (parts[0], parts[1])
}

fn hit(pos: &Position, t: &Target) -> bool {
    let (x, y) = t;
    ((x.0..=x.1).contains(&pos.re)) && ((y.0..=y.1).contains(&pos.im))
}

fn can_hit(pos: &Position, delta: &Position, t: &Target) -> bool {
    let (x, y) = t;

    !(pos.re > x.1) && ((pos.im >= y.0) || (delta.im >= 0))
}
fn adj_delta(delta: &Position) -> Position {
    let (x, y) = (delta.re, delta.im);
    Position::new(cmp::max(x - 1, 0), y - 1)
}

fn shoot(mut trajectory: Position, t: &Target) -> Option<Num> {
    let mut pos = Position::new(0, 0);
    let mut highest: Num = 0;
    loop {
        highest = cmp::max(highest, pos.im);
        if hit(&pos, t) {
            return Some(highest);
        }
        if !can_hit(&pos, &trajectory, t) {
            break;
        }
        pos += trajectory;
        trajectory = adj_delta(&trajectory);
    }
    return None;
}

fn y_at_step(y: Num, step: Num) -> Num {
    y * (step + 1) - (step * (step + 1)) / 2
}
fn limit(x:Num) -> Num {
    (x * (x + 1)) / 2

}

fn x_could_reach(t: &Target) -> HashMap<Num, HashSet<Num>> {
    let (x_, y_) = t;
    let t_ = (*x_, (Num::MIN, 0));

    let lower_limit = ((t.0 .0 * 2) as f64).sqrt() as Num;
    let mut res = HashMap::new();
    for x in (lower_limit..=t.0 .1) {
        let mut pos = Position::new(0, 0);
        let mut vel = Position::new(x, 0);
        for i in (0.. x + t.0.0.abs()*  10 ) {
            pos += vel;
            vel = adj_delta(&vel);
            if hit(&pos, &t_) {
                let mut vals = res.entry(x).or_insert(HashSet::new());
                vals.insert(i);
            }
        }
    }

    res
}
fn all_y_could_reach(t: &Target) -> HashMap<Num, HashSet<Num>> {
    let (x_, y_) = t;
    let t_ = ((Num::MIN, 0), *y_);
    let lower_limit = cmp::min(0, y_.0);
    let upper_limit = cmp::max(0, y_.1.abs() );
    println!("{:?}", (lower_limit, upper_limit));

    let mut res = HashMap::new();
    for y in (lower_limit..=upper_limit) {
        let mut pos = Position::new(0, 0);
        let mut vel = Position::new(0, y);
        for i in (0..upper_limit*2) {
            pos += vel;
            vel = adj_delta(&vel);
            if hit(&pos, &t_) {
                let mut vals = res.entry(y).or_insert(HashSet::new());
                vals.insert(i);
            }
        }
    }

    res}


fn y_could_reach(t: &Target, step: Num) -> HashSet<Num> {
    let (x_, y_) = t;
    let t_ = ((Num::MIN, 0), *y_);

    let lower_limit = cmp::min(0, y_.0);
    let mut res = HashSet::new();
    for y in (lower_limit..=cmp::max(step, y_.1.abs() )*10) {
        let pos = Position::new(0, y_at_step(y, step));

        if hit(&pos, &t_) {
            res.insert(y);
        }
    }

    res
}

fn highest_shot(t: &Target) -> Num {
    let mut heights: HashSet<Num>  = HashSet::new();
    for step in x_could_reach(&t).into_values().flatten().collect::<HashSet<Num>>().into_iter() { 

       let  ys = y_could_reach(t, step );
        for y in ys {

            heights.insert(y_at_step( y, y ) );
        }

    }
    heights.into_iter().max().unwrap()
}

fn num_shots(t: &Target) -> Num {
    let mut shots: HashSet<Position>  = HashSet::new();
  for (x, steps) in x_could_reach(&t).iter() { 
  //      for (x, steps) in HashMap::from([ (6, vec![4,5] ) ]) .iter() {
             
            for step in steps.into_iter() {

            for y in y_could_reach(t, *step ) {
                shots.insert(    Position::new(*x,y));
            } 
        }
    }
 //   shots.sort_by_key(|c| (c.re, c.im) );

   // println!("{:?}", shots);

    shots.len() as Num

}

pub fn part1() -> Num {
    let target = get_data();

    let x = x_could_reach(&target);
    println!("{:?}", x);


    highest_shot(&target)

}
pub fn part2() -> Num {
    let target = get_data();

    let y = all_y_could_reach(&target);
    println!("{:?}", y);
    num_shots(&target)
}

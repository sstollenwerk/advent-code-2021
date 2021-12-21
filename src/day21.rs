use crate::lib::to_filename;
use crate::lib::transpose;

use std::fs;

use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

use cached::proc_macro::cached;
use num_complex::Complex;

type Num = u64;

type Position = Num;
type Score = Num;
type State = (Position, Score )

fn ceilmod(n: Num, mod_: Num) -> Num {
    let k = n % mod_;
    if k == 0 {
        mod_
    } else {
        k
    }
}

fn get_data() -> (Num, Num) {
    let res: Vec<Num> = fs::read_to_string(to_filename(21))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect();

    (res[0], res[1])
}

fn read_row(row: &str) -> Num {
    row.chars().last().unwrap().to_digit(10).unwrap().into()
}

fn game(p1_:Num, p2_:Num) -> Num {
    let mut data = vec![(p1_, 0), (p2_, 0) ];
    // (position, score)
    let mut dice = (1..=100).cycle();
    let mut rolled = 0;
    // am sure there's an efficient way to do this
    loop {
        let (pos, score) = data.remove(0);
        let mut move_ = 0;
        for _ in (0..3) {
            move_ += dice.next().unwrap();
            rolled += 1;

        }
        let pos = ceilmod(pos+ move_, 10);
        let score = score + pos;
        if score >= 1000 {
        break 
        }
        else {
            data.push( (pos, score)  )
        }

    }
    (rolled * (data.pop().unwrap().1  )  )

}

pub fn part1() -> Num {
    let vals = get_data();
    println!("{:?}", vals);
    game(vals.0, vals.1)
   // todo!();
}
pub fn part2() -> Num {
    todo!();
}

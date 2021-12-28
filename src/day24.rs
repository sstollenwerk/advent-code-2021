use crate::lib::to_filename;
use crate::lib::transpose;

use std::cmp;
use std::fs;
use std::iter::from_fn;

use std::collections::HashMap;
use std::collections::HashSet;

use itertools::{iproduct, Itertools};

type Num = i64;

#[derive(Debug, Clone, std::cmp::PartialEq, Copy)]
enum Instruction {
    On,
    Off,
}

type Row = Vec<String>;
type Program = Vec<Row>;

type Inputs = Vec<Num>;
fn get_data() -> Vec<Row> {
    fs::read_to_string(to_filename(24))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect()
}

fn read_row(row: &str) -> Row {
    let parts: Vec<_> = row.split(' ').map(|s| s.to_string()).collect();
    parts
}

fn interpret(prog: &Program, inputs: &Vec<Num>) -> Vec<Num> {
    let mut data = HashMap::from([("w", 0), ("x", 0), ("y", 0), ("z", 0)]);
    let mut inp = inputs.clone().into_iter();

    for r in prog.iter() {
        let inst = &r[0][..];
        let m = &r[1][..];
        let a = data[m];
        let b: Num = match r.len() {
            2 => inp.next().unwrap(),
            3 => r[2].parse().unwrap_or_else(|_| data[&r[2][..]]),
            _ => panic!(),
        };

        let k = match inst {
            "inp" => {
			
			
			b},
            "add" => a + b,
            "mul" => a * b,
            "div" => a / b,
            "mod" => a % b,
            "eql" => (a == b) as Num,

            _ => panic!(),
        };
        let val = data.entry(m).or_insert(0);
        *val = k;
	    //println!("{:?}", r);
		//let m = [ data[&"w"], data[&"x"], data[&"y"], data[&"z"] ];
	    //println!("{:?}", m);
    }
	let m = [ data[&"w"], data[&"x"], data[&"y"], data[&"z"] ];
	//    println!("{:?}", m);

    m.to_vec()
}

fn prev(p_: &Inputs) -> Inputs {

	// [13] +1 = [12]
    let mut p = p_.clone();
    let mut i = 0;
	
	let digits = [12,11,9,8,7,6,5,4,3,2,1,0].to_vec();
	
    loop {
		let k = digits[i];
        if p[k] == 1 {
            p[k] = 9;
            i += 1;
        } else {
            p[k] -= 1;
            break;
        }
    }
	p[13] = p[12] - 1;
	p[10] = p[9] - 1;
	
	
	if p.contains(&0) { p = prev(&p)}
    p
}

pub fn part1() -> Num {
    let data = get_data();
    println!("{:?}", data);
   // let mut item = vec![9; 14];
 let mut item = vec![9; 14];
	item[13] = 10;
//	item[1] = 1;
	item[0] = 9;

    let items = from_fn(move || {
	item = prev(&item);
	Some(item.clone())
}
	);
	
	let mut prev_ = [-1, -1, -1, -1].to_vec();
	
	

    let use_ = items;
    for i in use_.take(1100) {
        let k = interpret(&data, &i);
		if prev_ != k {
		 println!("{:?}", i);
		 println!("{:?}", k);
		prev_ = k.clone();
		}
		if k[3] == 0 {
		let res = i.iter().map(|d| char::from_digit((*d).try_into().unwrap()  , 10).unwrap() ).collect::<String>();
			 println!("{:?}", res);
		
		break}
		
    }
	

	
    todo!();
}

pub fn part2() -> Num {
    todo!();
}

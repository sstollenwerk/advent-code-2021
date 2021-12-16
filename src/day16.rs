use crate::lib::to_filename;

use std::fs;

use itertools::{Either, Itertools};

type Version = u64;
type ID = u64;
type Literal = u64;

#[derive(Debug, Clone)]
struct Packet {
    ver: Version,
    id: ID,
    data: Either<Literal, Vec<Packet>>,
}

impl Packet {
    fn new(ver: Version, id: ID, data: Either<Literal, Vec<Packet>>) -> Packet {
        Packet { ver, id, data }
    }
}

fn get_data() -> Vec<bool> {
    fs::read_to_string(to_filename(16))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .next()
        .unwrap()
}

fn read_row(row: &str) -> Vec<bool> {
    row.chars()
        .map(|d| d.to_digit(16).unwrap())
        .map(|n| format!("{:04b}", n).chars().collect::<Vec<char>>())
        .flatten()
        .map(|d| d.to_digit(2).unwrap() != 0)
        .collect::<Vec<_>>()
}

fn as_base(data: &Vec<Literal>, base: Literal) -> Literal {
    //  println!("{:?}", ( data, base) );
    let mut res = 0;
    for i in data {
        res *= base;
        res += i;
    }
    res
}

fn from_bin(n: &[bool]) -> Literal {
    as_base(&n.iter().map(|b| *b as Literal).collect(), 2)
}

fn parse(data: &[bool]) -> Vec<Packet> {
    // println!("{:?}", data);
    if data.is_empty() || data.len() < 8 {
        return Vec::new();
    }
    let (ver, rest) = data.split_at(3);
    let ver = from_bin(ver);
    let (id, rest) = rest.split_at(3);
    let id = from_bin(id);
    if id == 4 {
        let mut i = 0;
        let mut num = Vec::new();
        let iter = rest.chunks(5);
        for c in iter {
            let (a, b) = c.split_at(1);
            num.append(&mut b.to_vec());
            i += 5;
            if !a[0] {
                break;
            }
        }
        let n = from_bin(&num[..]);
        let rest: &[bool] = &rest[i..];

        let mut res = vec![Packet::new(ver, id, itertools::Either::Left(n))];
        res.append(&mut parse(rest));
        return res;
    } else {
        let (a, rest) = rest.split_at(1);
        let a = a[0];

        if !a {
            let (size, rest) = rest.split_at(15);
            let size = from_bin(size);
            let (contained, rest) = rest.split_at(size as usize);

            let mut res = vec![Packet::new(
                ver,
                id,
                itertools::Either::Right(parse(contained)),
            )];
            res.append(&mut parse(rest));
            return res;
        } else {
            let (size, rest) = rest.split_at(11);
            let size = from_bin(size);
            let part = parse(rest);
            let (contained, rest) = part.split_at(size as usize);
            let mut res = vec![Packet::new(
                ver,
                id,
                itertools::Either::Right(contained.to_vec()),
            )];
            res.append(&mut rest.to_vec());
            return res;
        }
    }
}
fn version_sum(packets: Vec<Packet>) -> Version {
    let mut total: Version = 0;
    for p in packets.into_iter() {
        let data = p.data;
        total += p.ver;
        total += match data {
            itertools::Either::Right(d) => version_sum(d),
            itertools::Either::Left(_) => 0,
        }
    }
    total
}

fn interpret(packet: Packet) -> Literal {
    let vals : Vec<Literal> = packet.data.clone().right().unwrap_or(vec![]).into_iter().map(|p| interpret(p)).collect();
    let a = vals.iter();
    let res = match packet.id {
        4 => packet.data.left().unwrap(),

        0 => a.sum(),
        1 => a.product(),
        2 => *a.min().unwrap(),
        3 => *a.max().unwrap(),
        5 => (vals[0] > vals[1] ) as Literal,
        6 => (vals[0] < vals[1] ) as Literal,
        7 => (vals[0] == vals[1] ) as Literal,
        _ => panic!(),
    };
    res
}

pub fn part1() -> Version {
    let row = get_data();
    //   println!("{:?}", row);
    let res = parse(&row[..]);
    // println!("{:?}",res );

    version_sum(res)
}
pub fn part2() -> Literal {
    let row = get_data();
    let res = parse(&row[..]);

    res.into_iter().map(|p| interpret(p)).sum()
}

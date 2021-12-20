use crate::lib::to_filename;

use std::fs;

type Num = u64;

// use trees::{tr, Node, Tree};

//type SnailNum = trees::Tree<Option<Num>>;
//type SnailNum = trees::Tree<Num>;
// Option can't be printed in trees - will change once I'm sure it works

#[derive(Debug, Clone)]
enum SnailNum {
    Leaf(Num),
    Node(Vec<SnailNum>),
}

#[derive(Debug, Clone, std::cmp::PartialEq, Copy)]
enum Piece {
    Open,
    Close,
    Val(Num),
}
type SnailNum_ = Vec<Piece>;

fn get_data() -> Vec<SnailNum> {
    fs::read_to_string(to_filename(18))
        .expect("Could not read file")
        .lines()
        .map(|s| shrink(read_row(s)))
        .collect::<Vec<_>>()
}

fn get_data_() -> Vec<SnailNum_> {
    fs::read_to_string(to_filename(18))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row_(s))
        .collect::<Vec<_>>()
}

fn shrink(s: SnailNum) -> SnailNum {
    if let SnailNum::Node(ref xs) = s {
        if xs.len() == 1 {
            return shrink(xs[0].clone());
        }
    }
    return s;
}
fn read_row_(row: &str) -> SnailNum_ {
    row.chars()
        .filter_map(|c| match c {
            '[' => Some(Piece::Open),
            ']' => Some(Piece::Close),
            ',' => None,
            n => Some(Piece::Val(n.to_digit(10).unwrap().into())),
        })
        .collect()
}

fn read_row(row: &str) -> SnailNum {
    println!("{:?}", row);

    let mut node = SnailNum::Node(Vec::new());

    let mut amt = 0;
    let mut pos = 0;
    for (i, c) in row.chars().enumerate() {
        match c {
            '[' => {
                if amt == 0 {
                    pos = i + 1
                }
                amt += 1
            }
            ']' => amt -= 1,
            ',' => amt -= 0,
            n => {
                if amt == 0 {
                    if let SnailNum::Node(ref mut xs) = node {
                        xs.push(SnailNum::Leaf(n.to_digit(10).unwrap() as Num))
                    }
                }
            }
        }
        if amt == 0 && c == ']' {
            if let SnailNum::Node(ref mut xs) = node {
                xs.push(read_row(&row[pos..i]));
            }
            pos = i;
        }
    }
    return node;
}

/*fn read_part(mut node: SnailNum, row: &str) -> SnailNum {
    if !row.contains('[') {
        for n in row.chars().filter_map(|s| s.to_digit(10)).map(|n| n as Num) {
            node.push_back(Tree::new(Some(n)))
        }
        return node;
    } else {
        let mut amt = 0;
        let mut pos = 0;
        for (i, c) in row.chars().enumerate() {
            match c {
                '[' => amt += 1,
                ']' => amt -= 1,
                _ => amt -= 0,
            }
            if amt == 0 {
                node = (read_part(node, &row[pos..i]));
                pos = i;
            }
        }
        return node;
    }
}*/
fn split(num: SnailNum) -> SnailNum {
    match num {
        SnailNum::Node(xs) => SnailNum::Node(vec![split(xs[0].clone()), split(xs[1].clone())]),
        SnailNum::Leaf(n) => {
            if n < 10 {
                SnailNum::Leaf(n)
            } else {
                SnailNum::Node(vec![
                    SnailNum::Leaf(n / 2),
                    SnailNum::Leaf((n % 2) + (n / 2)),
                ])
            }
        }
    }
}

fn magnitude(num: &SnailNum) -> Num {
    match num {
        SnailNum::Leaf(n) => *n,
        SnailNum::Node(xs) => 3 * magnitude(&xs[0]) + 2 * magnitude(&xs[1]),
    }
}
fn add(mut a: SnailNum_, mut b: SnailNum_) -> SnailNum_ {
    a.insert(0, Piece::Open);
    a.append(&mut b);
    a.push(Piece::Close);
    a
}

fn reduce_(mut s: SnailNum_) -> SnailNum_ {
    let mut depth = 0;
    let mut i = 0;
    let mut restart = false;
    while i < s.len() {
        if restart {
            depth = 0;
            i = 0;
            restart = false;
        }
        println!("{:?}", depth);
        println!("{:?}", i);
        println!("{:?}", &s);
        println!("{:?}", s[i]);
        println!("");

        if let Piece::Val(n) = s[i] {
            if depth > 4 {
                assert_eq!(s[i - 1], Piece::Open);
                assert_eq!(s[i + 2], Piece::Close);
                let a = &mut s[..i - 1].to_vec();
                let b = &mut s[i + 3..].to_vec();
                let mut x = 0;
                let mut y = 0;
                if let Piece::Val(x_) = s[i] {
                    x = x_
                } else {
                    panic!();
                };
                if let Piece::Val(y_) = s[i + 1] {
                    y = y_
                } else {
                    panic!();
                };

                for j in (0..b.len()) {
                    if let Piece::Val(n) = b[j] {
                        b[j] = Piece::Val(y + n);
                        restart = true;
                        break;
                    }
                }

                for j in (0..(a.len())).rev() {
                    if let Piece::Val(n) = a[j] {
                        a[j] = Piece::Val(x + n);
                        restart = true;
                        break;
                    }
                }

                a.push(Piece::Val(0));
                a.append(b);
                s = a.to_vec();
                depth -= 1;
            }

            if n > 10 {
                let part = &mut vec![
                    Piece::Open,
                    Piece::Val((n / 2)),
                    Piece::Val((n % 2) + (n / 2)),
                    Piece::Close,
                ];
                let a = &mut s[..i].to_vec();
                let b = &mut s[i + 1..].to_vec();
                a.append(part);
                a.append(b);
                s = a.to_vec();
                restart = true;
                continue;
            }
        } else if Piece::Open == s[i] {
            depth += 1
        } else if s[i] == Piece::Close {
            depth -= 1
        }

        i += 1;
    }
    println!("{:?}", s);

    s
}

pub fn part1() -> Num {
    let res = get_data_()
        .into_iter()
        .reduce(|a, b| reduce_(add(a, b)))
        .unwrap();
    println!("{:?}", res);

    todo!();
}
pub fn part2() -> Num {
    todo!();
}

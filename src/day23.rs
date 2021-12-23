use crate::lib::{sorted, to_filename};

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use std::ops::Add;

use itertools::Itertools;

use num_complex::Complex;

use pathfinding::prelude::{absdiff, astar, dijkstra};

use lazy_static::lazy_static;

type Tile = char;

type Num = i32;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct Place {
    x: Num,
    y: Num,
}

impl Add for Place {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Place {
    fn new(x: Num, y: Num) -> Place {
        Place { x, y }
    }
}

type Grid = HashMap<Place, Tile>;

type AmphLocs = Vec<Vec<Place>>;

type Places = Vec<Place>;
type Index = (usize, usize);

fn get_data() -> (AmphLocs, Places) {
    let mut grid: Grid = HashMap::new();
    let mut places: HashMap<Tile, Vec<Place>> = HashMap::new();

    for (i, row) in fs::read_to_string(to_filename(23))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .enumerate()
    {
        for (j, c) in row.iter().enumerate() {
            let p = Place::new(i as Num, j as Num);
            grid.insert(p, *c);
            let k = places.entry(*c).or_insert(Vec::new());
            (*k).push(p);
            (*k).sort();
        }
    }
    //  println!("{:?}", &grid);
    //  println!("");
    //   println!("{:?}", &places);

    let amphs = vec![
        places[&'A'].clone(),
        places[&'B'].clone(),
        places[&'C'].clone(),
        places[&'D'].clone(),
    ];

    let mut can_move_on = vec![amphs.clone(), vec![places[&'.'].clone()]]
        .concat()
        .concat();

    can_move_on.sort();

    //   println!("");
    //   println!("{:?}", &amphs);
    //  println!("");
    //    println!("{:?}", &can_move_on);

    (amphs, can_move_on)
}

fn read_row(row: &str) -> Vec<Tile> {
    row.chars().collect::<Vec<_>>()
}

lazy_static! {
    static ref CAN_MOVE_ON:Places = get_data().1 ;
   // static ref n: Num = 5;
}

fn temp_allowed_positions(amphs: &AmphLocs) -> HashSet<Place> {
    let mut cant_move_to = amphs.concat();

    let cant_move_to: HashSet<Place> = cant_move_to.iter().map(|x| *x).collect::<HashSet<_>>();
    (*CAN_MOVE_ON)
        .iter()
        .map(|x| *x)
        .collect::<HashSet<_>>()
        .difference(&cant_move_to)
        .map(|x| *x)
        .collect()
    //   println!("{:?}", &can_move_to);
}

fn allowed_positions(amphs: &AmphLocs) -> HashSet<Place> {
    let mut cant_move_to = amphs.concat();
    cant_move_to.append(&mut vec![
        Place::new(1, 3),
        Place::new(1, 5),
        Place::new(1, 7),
        Place::new(1, 9),
    ]);
    let cant_move_to: HashSet<Place> = cant_move_to.iter().map(|x| *x).collect::<HashSet<_>>();
    (*CAN_MOVE_ON)
        .iter()
        .map(|x| *x)
        .collect::<HashSet<_>>()
        .difference(&cant_move_to)
        .map(|x| *x)
        .collect()
    //   println!("{:?}", &can_move_to);
}

fn amph_allowed_move_to(amphs: &AmphLocs, a: Index) -> HashSet<Place> {
    let mut poss = allowed_positions(amphs);
    let hallway = (a.0) * 2 + 3;
    poss.retain(|p| p.x == 1 || p.y == hallway.try_into().unwrap());

    let hall: HashSet<Place> = poss
        .iter()
        .map(|x| *x)
        .filter(|p| p.x != 1 && p.y == hallway.try_into().unwrap())
        .collect();

    let mut occupied: bool = false;
    for (i, group) in amphs.iter().enumerate() {
        for c in group.iter() {
            if i != a.0.try_into().unwrap() && hall.contains(c) {
                occupied = true;
            }
        }
    }
    if occupied {
        poss = poss.difference(&hall).map(|x| *x).collect();
    }
    poss
}

fn heuristic(amphs: &AmphLocs) -> Num {
    let mut score = Vec::new();
    for (i, group) in amphs.iter().enumerate() {
        for (j, c) in group.iter().enumerate() {
            let move_cost = (10 as Num).pow(i.try_into().unwrap());
            //  let move_cost =( i+1) as i32;
            //    let move_cost =( 1) as i32;
            let hallway = (i) * 2 + 3;
            let dist = ((c.y as i32) - (hallway as i32)).abs();
            score.push(dist * move_cost)
        }
    }
    score.iter().sum()
}

fn poss_moves(amphs: &AmphLocs) -> Vec<(AmphLocs, Num)> {
    // for amph in amph locs
    // for possible allowed_position
    // get shortest path there if it exists.
    let mut res = Vec::new();

    for (i, group) in amphs.iter().enumerate() {
        for (j, c) in group.iter().enumerate() {
            // let posses = amph_allowed_move_to(amphs, (i, j));
            //  let move_cost = (10 as Num).pow(i.try_into().unwrap());
            res.append(&mut amph_move(amphs, (i, j)))
        }
    }

    res
}

fn sort_state(amphs: &AmphLocs) -> AmphLocs {
    let mut state = amphs.clone();
    for i in (0..state.len()) {
        state[i].sort();
    }
    state
}

fn amph_move(amphs: &AmphLocs, a: Index) -> Vec<(AmphLocs, Num)> {
    let mut res = Vec::new();
    let move_cost = (10 as Num).pow(a.0.try_into().unwrap());
    for poss in amph_allowed_move_to(amphs, a) {
        let success = |loc: &AmphLocs| loc[a.0][a.1] == poss;
        let successors = |loc: &AmphLocs| sing_move_adj_state(loc, a);

        let p = dijkstra(amphs, successors, success);

        if let Some(path) = p {
            //     println!("{:?}", poss);
            //    println!("{:?}", res);
            let state = path.0[path.1 as usize].clone();
            let state = sort_state(&state);

            res.push((state, path.1 * move_cost))
        }
    }

    res
}

fn sing_move_adj_state(amphs: &AmphLocs, a: Index) -> Vec<(AmphLocs, Num)> {
    let mut res = Vec::new();
    let p = amphs[a.0][a.1];
    for adj in adjacent_places(amphs, p) {
        let mut new_state = amphs.clone();
        new_state[a.0][a.1] = adj;
        res.push((new_state, 1));
    }
    res
}

fn adjacent_places(amphs: &AmphLocs, pos: Place) -> Vec<Place> {
    let allowed = temp_allowed_positions(amphs);
    let nears = [
        Place::new(1, 0),
        Place::new(-1, 0),
        Place::new(0, 1),
        Place::new(0, -1),
    ];
    nears
        .iter()
        .map(|c| pos + *c)
        .filter(|c| allowed.contains(&c))
        .collect()
}

fn display(amphs: &AmphLocs) -> () {
    let size = amphs.concat().iter().map(|c| c.x).max().unwrap() + 3;
    let mut row = ['#'; 12].to_vec();
    let mut res = Vec::new();
    for _ in (0..size) {
        res.push(row.clone());
    }

    for c in (*CAN_MOVE_ON).iter().map(|x| *x) {
        res[c.x as usize][c.y as usize] = '.';
    }
    //[row.clone(); 5].to_vec();
    for (i, group) in amphs.iter().enumerate() {
        for (j, c) in group.iter().enumerate() {
            res[c.x as usize][c.y as usize] = vec!['A', 'B', 'C', 'D'][i];
        }
    }

    for i in res {
        println!("{:?}", i.iter().cloned().collect::<String>());
    }
    println!("");
}

pub fn part1() -> Num {
    let (amphs, _) = get_data();

    println!("{:?}", amphs);

    let desired = sort_state(&vec![
        vec![Place { x: 2, y: 3 }, Place { x: 3, y: 3 }],
        vec![Place { x: 2, y: 5 }, Place { x: 3, y: 5 }],
        vec![Place { x: 2, y: 7 }, Place { x: 3, y: 7 }],
        vec![Place { x: 2, y: 9 }, Place { x: 3, y: 9 }],
    ]);

    println!("{:?}", heuristic(&desired));
    println!("{:?}", heuristic(&amphs));

    if let Some(res) = astar(&amphs, poss_moves,heuristic,  |a| *a == desired) {
    //if let Some(res) = dijkstra(&amphs, poss_moves, |a| *a == desired) {
        let (a, b) = res;
        for i in a {
            display(&i);
        }
        println!("{}", b);
        return b
    }
    //    println!("{:?}", astar(&amphs, poss_moves,heuristic,  |a| *a == desired));

    panic!();
}
pub fn part2() -> Num {
    let (amphs, _) = get_data();

    println!("{:?}", amphs);

    let desired = sort_state(&vec![
        vec![Place { x: 2, y: 3 }, Place { x: 3, y: 3 }, Place { x: 4, y: 3 }, Place { x: 5, y: 3 }],
        vec![Place { x: 2, y: 5 }, Place { x: 3, y: 5 },Place { x: 4, y: 5 }, Place { x: 5, y: 5 },],
        vec![Place { x: 2, y: 7 }, Place { x: 3, y: 7 },Place { x: 4, y: 7 }, Place { x: 5, y: 7 },],
        vec![Place { x: 2, y: 9 }, Place { x: 3, y: 9 },Place { x: 4, y: 9 }, Place { x: 5, y: 9 },],
    ]);


    if let Some(res) = astar(&amphs, poss_moves,heuristic, |a| *a == desired) {
        let (a, b) = res;
        for i in a {
            display(&i);
        }
        println!("{}", b);
        return b
    }

    panic!();

}

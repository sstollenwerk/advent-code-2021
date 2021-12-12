use crate::lib::to_filename;

use std::fs;

use std::collections::HashMap;

use counter::Counter;

type Node = String;

type Edge = (Node, Node);

type Digraph = HashMap<Node, Vec<Node>>;
type Path = Vec<Node>;

type Res = u32;

fn get_data() -> Digraph {
    to_digraph(
        fs::read_to_string(to_filename(12))
            .expect("Could not read file")
            .lines()
            .map(|s| read_row(s))
            .collect(),
    )
}

fn to_digraph(rows: Vec<Edge>) -> Digraph {
    let mut groups = HashMap::new();
    for (start, end) in rows.into_iter() {
        let vals = groups.entry(start.clone()).or_insert(Vec::new());
        vals.push(end.to_string());

        let vals2 = groups.entry(end.clone()).or_insert(Vec::new());
        vals2.push(start.to_string());
    }
    groups
}

fn read_row(row: &str) -> Edge {
    let data = row.split('-').collect::<Vec<_>>();
    (data[0].to_string(), data[1].to_string())
}

fn paths(graph: &Digraph, start: &Node, end: &Node) -> Res {
    if start == end {
        return 1;
    }
    let mut res: Res = 0;

    let nexts = graph.get(&start.clone()).unwrap_or(&Vec::new()).to_vec();

    let mut g = graph.clone();

    if start == &start.to_ascii_lowercase() {
        g.remove(start);

        // making it impossible to leave this node after reaching a second time.
        // not sure if that'll work
    }
    for n in nexts.iter() {
        let paths_ = paths(&g, n, &end.clone());
        res += paths_
    }

    res
}

fn is_lowercase(s: &String) -> bool {
    s == &s.to_ascii_lowercase()
}

fn valid(p: &Path) -> bool {
    let amts = p
        .into_iter()
        .filter(|s| is_lowercase(s))
        .collect::<Counter<&Node>>();

    let cnt_lowers = amts.values().collect::<Counter<&usize>>();

    (amts.get(&"start".to_string()).unwrap_or(&0) <= &1)
        && (amts.get(&"end".to_string()).unwrap_or(&0) <= &1)
        && (cnt_lowers[&2] <= 1)
        && (cnt_lowers.keys().max().unwrap_or(&&0) <= &&2)
}

fn paths2(graph: &Digraph, start: &Node, end: &Node, prev: &Path) -> Vec<Path> {
    if start == end {
        return vec![vec![start.to_string()]];
    }
    let mut res: Vec<Path> = Vec::new();

    if !valid(&prev) {
        return res;
    }

    let nexts = graph.get(&start.clone()).unwrap_or(&Vec::new()).to_vec();

    let mut p = prev.to_vec();
    p.push(start.to_string());

    for n in nexts.iter() {
        let mut paths_: Vec<Path> = paths2(graph, n, &end.clone(), &p);
        for r in paths_.iter_mut() {
            r.push(start.to_string());
            if valid(r) {
                res.push(r.to_vec());
            }
        }
    }
    res
}

pub fn part1() -> Res {
    let graph = get_data();
    println!("{:?}", graph);
    paths(&graph, &"start".to_string(), &"end".to_string())
}

pub fn part2() -> Res {
    let graph = get_data();
    paths2(
        &graph,
        &"start".to_string(),
        &"end".to_string(),
        &Vec::new(),
    )
    .len() as Res
}

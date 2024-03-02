pub const INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
extern crate nalgebra as na;

use std::collections::{HashMap, HashSet};

use na::DMatrix;

pub fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    let graph: HashMap<String, HashSet<String>> = input
        .trim()
        .lines()
        .map(|line| {
            let (key, vals) = line.split_once(":").unwrap();
            (
                key.to_string(),
                vals.trim()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
            )
        })
        .collect();
    let mut reverse = graph.clone();
    for (key, vals) in graph {
        for val in vals {
            reverse.entry(val).or_default().insert(key.clone());
        }
    }
    reverse
}

pub fn part_one(input: &str) -> usize {
    let graph = parse(input);
    let index: HashMap<_, _> = graph
        .keys()
        .enumerate()
        .map(|(i, k)| (k.clone(), i))
        .collect();

    let mut mat: DMatrix<usize> = DMatrix::zeros(index.len(), index.len());
    for (key, vals) in graph {
        let i = *index.get(&key).unwrap();
        for val in vals {
            let j = *index.get(&val).unwrap();
            mat[(i, j)] = 1;
        }
    }
    println!("{}", mat);

    0
}

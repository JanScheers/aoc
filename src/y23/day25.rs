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
use rand::Rng;

use std::collections::{HashMap, HashSet, VecDeque};

pub fn parse(input: &str) -> Vec<Vec<usize>> {
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
    let mut rev = graph.clone();
    for (key, vals) in graph {
        for val in vals {
            rev.entry(val).or_default().insert(key.clone());
        }
    }
    let to_index: HashMap<_, _> = rev
        .keys()
        .enumerate()
        .map(|(i, k)| (k.clone(), i))
        .collect();
    rev.values()
        .map(|v| v.iter().map(|v| *to_index.get(v).unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let graph = parse(input);
    let len = graph.len();
    let mut freqs = na::DMatrix::zeros(len, len);
    for i in 1.. {
        println!("{}", i);
        random_walk(&graph, &mut freqs);
        for parts in cluster(&freqs) {
            if let Some(_graph) = cut(&graph, &parts) {
                return parts.iter().map(|p| p.len()).product();
            }
        }
    }
    0
}

fn random_walk(graph: &Vec<Vec<usize>>, freqs: &mut na::DMatrix<usize>) {
    let graph_len = graph.len();
    let walk_len = 150.min(graph_len);
    let mut rng = rand::thread_rng();
    for mut head in 0..graph_len {
        let mut walk = VecDeque::with_capacity(walk_len);
        let mut freq = na::DVector::zeros(graph_len);
        for _ in 0..walk_len {
            head = graph[head][rng.gen_range(0, graph[head].len())];
            walk.push_back(head);
            freq[head] += 1;
        }

        let mut col = freqs.column(head) + &freq;
        freqs.column_mut(head).copy_from(&col);
        for _ in 0..walk_len {
            head = graph[head][rng.gen_range(0, graph[head].len())];
            walk.push_back(head);
            freq[head] += 1;
            freq[walk.pop_front().unwrap()] -= 1;

            freqs.column(head).add_to(&freq, &mut col);
            freqs.column_mut(head).copy_from(&col);
        }
    }
}

fn cluster(freqs: &na::DMatrix<usize>) -> Vec<Vec<HashSet<usize>>> {
    let count = freqs
        .column_iter()
        .fold(HashMap::new(), |mut count: HashMap<_, usize>, col| {
            for parts in cluster_col(col.iter()) {
                *count.entry(parts).or_default() += 1;
            }
            count
        });
    let mut count: Vec<_> = count.into_iter().collect();
    count.sort_unstable_by_key(|c| c.1);
    count.reverse();
    count[..3]
        .iter()
        .map(|parts| {
            parts
                .0
                .iter()
                .map(|part| part.iter().map(|u| *u).collect())
                .collect()
        })
        .collect()
}

fn cluster_col<'a>(col: impl Iterator<Item = &'a usize>) -> Vec<Vec<Vec<usize>>> {
    let mut cs: Vec<_> = col.enumerate().map(|(i, c)| (*c, i)).collect();
    cs.sort();
    let part = (1..cs.len() - 1)
        .map(|part| (std(&cs[..part]) + std(&cs[part..]), part))
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        .1;
    (part.saturating_sub(4)..(part + 4).min(cs.len() - 1))
        .map(|part| {
            let mut parts: Vec<_> = vec![&cs[..part], &cs[part..]]
                .iter()
                .map(|cs| {
                    let mut is: Vec<_> = cs.iter().map(|c| c.1).collect();
                    is.sort();
                    is
                })
                .collect();
            parts.sort_by_key(|v| v.len());
            parts
        })
        .collect()
}

fn std(cs: &[(usize, usize)]) -> f64 {
    na::DVector::from_iterator(cs.len(), cs.iter().map(|c| c.0 as f64))
        .variance()
        .sqrt()
}

fn cut(graph: &Vec<Vec<usize>>, parts: &Vec<HashSet<usize>>) -> Option<Vec<Vec<usize>>> {
    let mut cuts = 0;
    let graph = graph
        .iter()
        .enumerate()
        .map(|(node, verts)| {
            let part = parts.iter().find(|p| p.contains(&node)).unwrap();
            verts
                .iter()
                .filter_map(|v| {
                    if part.contains(v) {
                        Some(*v)
                    } else {
                        cuts += 1;
                        None
                    }
                })
                .collect()
        })
        .collect();
    if cuts == 6 {
        Some(graph)
    } else {
        None
    }
}

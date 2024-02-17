use std::collections::HashSet;

use num::traits::Pow;

use crate::{day1, size, Vec2, DIRS};

pub const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Black {
    even: i64,
    odd: i64,
    corners: i64,
    borders: i64,
}

struct White {
    center: i64,
    border: i64,
}

struct Formula {
    even: i64,
    odd: i64,
    center: i64,
    corner: Vec<i64>,
    on_border: Vec<i64>,
    off_border: Vec<i64>,
}

impl Formula {
    fn new() -> Self {
        Formula {
            even: 0,
            odd: 0,
            center: 0,
            corner: vec![],
            on_border: vec![],
            off_border: vec![],
        }
    }
}

pub fn parse(input: &str) -> (Vec2<i64>, Vec<Vec<bool>>) {
    let mut start = Vec2(-1, -1);
    let map = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => true,
                    'S' => {
                        start = Vec2(i as i64, j as i64);
                        true
                    }
                    _ => false,
                })
                .collect()
        })
        .collect();
    (start, map)
}

pub fn part_one(input: &str, nsteps: usize) {
    let (start, map) = parse(input);
    let (m, n) = size(&map);
    let (m, n) = (m as i64, n as i64);
    let mut frontier: HashSet<_> = HashSet::from([start]);
    for _ in 0..nsteps {
        let mut next: HashSet<Vec2<i64>> = HashSet::new();
        frontier.iter().for_each(|square| {
            next.extend(DIRS.iter().filter_map(|d| {
                let step = *square + *d;
                if 0 <= step.0
                    && step.0 < m
                    && 0 <= step.1
                    && step.1 < n
                    && map[step.0 as usize][step.1 as usize]
                {
                    Some(step)
                } else {
                    None
                }
            }))
        });
        frontier = next;
    }
    dbg!(frontier.len());
}

pub fn part_two(input: &str, nsteps: usize) -> i64 {
    let (start, map) = parse(input);
    let m = map.len() as i64;
    let mut frontier: HashSet<_> = HashSet::from([start]);
    let mut form = Formula::new();
    for st in 1.. {
        let mut next: HashSet<Vec2<i64>> = HashSet::new();
        frontier.iter().for_each(|square| {
            next.extend(DIRS.iter().filter_map(|d| {
                let step = *square + *d;
                if map[step.0.rem_euclid(m) as usize][step.1.rem_euclid(m) as usize] {
                    Some(step)
                } else {
                    None
                }
            }))
        });
        frontier = next;
        let (incnt, outcnt) = count(&frontier, m, st);
        let n = (st - m / 2 - 1) / m;
        if n == 1 {
            form.even = incnt[2][2 + (st + 1) as usize % 2];
            form.odd = incnt[2][2 + st as usize % 2];
            form.center = outcnt[1][1] + outcnt[1][2];
            form.on_border
                .push(incnt[1][1] + incnt[3][1] + incnt[1][3] + incnt[3][3]);
            form.corner
                .push(incnt[0][2] + incnt[2][0] + incnt[4][2] + incnt[2][4]);
            form.off_border
                .push(outcnt[0][1] + outcnt[0][2] + outcnt[3][1] + outcnt[3][2])
        }
        if n > 1 {
            break;
        }
    }
    let (m, half) = (m as usize, m as usize / 2);
    let formula = |st: usize| {
        let n = (st - half - 1).div_euclid(m) as i64;
        let i = (st - half - 1).rem_euclid(m);
        (n + 1) * n * form.center
            + n * form.on_border[i]
            + (n + 1) * form.off_border[i]
            + form.corner[i]
            + form.even * (n + (i as i64) % 2).pow(2)
            + form.odd * (n + (i as i64 + 1) % 2).pow(2)
    };
    formula(nsteps)
}

fn count(frontier: &HashSet<Vec2<i64>>, m: i64, st: i64) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let half = m / 2;
    let bottom = (half - st).div_euclid(m);
    let to_coord = |v: &Vec2<i64>| {
        let delta = (v.0.rem_euclid(m) - half).abs() + (v.1.rem_euclid(m) - half).abs();
        let inside = delta <= m / 2;
        (
            inside,
            ((if inside { v.0 } else { v.0 - half }).div_euclid(m) - bottom) as usize,
            ((if inside { v.1 } else { v.1 - half }).div_euclid(m) - bottom) as usize,
        )
    };
    let n = ((half + st).div_euclid(m) - bottom + 1) as usize;
    let mut incnt = vec![vec![0; n]; n];
    let mut outcnt = vec![vec![0; n - 1]; n - 1];
    for v in frontier {
        let (inside, i, j) = to_coord(v);
        if inside {
            incnt[i][j] += 1;
        } else {
            outcnt[i][j] += 1;
        }
    }
    (incnt, outcnt)
}

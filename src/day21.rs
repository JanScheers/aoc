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

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct Black {
    even: i64,
    odd: i64,
    corners: i64,
    borders: i64,
}

impl Black {
    fn new() -> Black {
        Black {
            even: 0,
            odd: 0,
            corners: 0,
            borders: 0,
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

pub fn part_two(input: &str, nsteps: usize) {
    let (start, map) = parse(input);
    let (m, n) = size(&map);
    let (m, n) = (m as i64, n as i64);
    let half = m / 2;
    let mut frontier: HashSet<_> = HashSet::from([start]);
    let (mut even, mut odd, mut corners, mut borders) = (0, 0, vec![], vec![]);
    for st in 1.. {
        let mut next: HashSet<Vec2<i64>> = HashSet::new();
        frontier.iter().for_each(|square| {
            next.extend(DIRS.iter().filter_map(|d| {
                let step = *square + *d;
                if map[step.0.rem_euclid(m) as usize][step.1.rem_euclid(n) as usize] {
                    Some(step)
                } else {
                    None
                }
            }))
        });
        frontier = next;
        let n = (st as i64 - half - 1).div_euclid(m);
        if n == 1 || n == 2 {
            let (incnt, outcnt) = count(&frontier, m, st);
            let d = in_diamond(&incnt, st);
            even = d.even;
            odd = d.odd;
            corners.push(d.corners);
            borders.push(d.borders);

            out_diamond(&outcnt, m, st)
        }
        if n > 2 {
            break;
        }
    }
    let (m, half) = (m as usize, half as usize);
    let formula = |st: usize| {
        let n = (st - half - 1).div_euclid(m) as i64;
        let i = (st - half - 1).rem_euclid(m);
        n * borders[i]
            + corners[i]
            + if i % 2 == 0 {
                even * (n + 1).pow(2) + odd * n.pow(2)
            } else {
                even * n.pow(2) + odd * (n + 1).pow(2)
            }
    };

    let (m, half) = (m as i64, half as i64);
    let mut frontier: HashSet<_> = HashSet::from([start]);
    for st in 1.. {
        let mut next: HashSet<Vec2<i64>> = HashSet::new();
        frontier.iter().for_each(|square| {
            next.extend(DIRS.iter().filter_map(|d| {
                let step = *square + *d;
                if map[step.0.rem_euclid(m) as usize][step.1.rem_euclid(n) as usize] {
                    Some(step)
                } else {
                    None
                }
            }))
        });
        frontier = next;

        let n = (st as i64 - half - 1).div_euclid(m);
        if n >= 0 {
            let (incnt, _) = count(&frontier, m, st);
            let a = incnt.iter().map(|v| v.iter().sum::<i64>()).sum::<i64>();
            let b = formula(st as usize);
            println!("{} {} {} {} {}", st, n, a, b, a == b);
        }
    }

    dbg!(even, odd, corners, borders);
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

fn in_diamond(incnt: &Vec<Vec<i64>>, st: i64) -> Black {
    return Black {
        even: incnt[2][2 + st as usize % 2],
        odd: incnt[2][2 + (st + 1) as usize % 2],
        corners: incnt[0][2] + incnt[2][0] + incnt[4][2] + incnt[2][4],
        borders: incnt[1][1] + incnt[3][1] + incnt[1][3] + incnt[3][3],
    };
}

fn out_diamond(outcnt: &Vec<Vec<i64>>, m: i64, st: i64);

pub fn draw(step: i64) {
    let m: i64 = 7;

    for i in -2 * m..3 * m {
        if i.rem_euclid(m) == 0 {
            println!("{}", vec!["--"; (5 * m) as usize + 2].join(""))
        }
        for j in -2 * m..3 * m {
            if j.rem_euclid(m) == 0 {
                print!("|");
            }
            if j.rem_euclid(m) == 0 {}
            let delta = (i.rem_euclid(m) - m / 2).abs() + (j.rem_euclid(m) - m / 2).abs();
            let inside = delta <= m / 2;
            let icoord = (i.div_euclid(m), j.div_euclid(m));
            let ocoord = ((i + m / 2).div_euclid(m), (j + m / 2).div_euclid(m));
        }
        print!("\n");
    }
}

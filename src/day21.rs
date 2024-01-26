use std::collections::HashSet;

use crate::{size, Vec2, DIRS};

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
    let mut frontier: HashSet<_> = HashSet::from([start]);
    show_walk(&map, &frontier, 1);
    for st in 1..nsteps {
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
        if st % 131 == 66 {
            show_walk(&map, &frontier, st)
        }
    }
    dbg!(frontier.len());
}

fn show_walk(map: &Vec<Vec<bool>>, frontier: &HashSet<Vec2<i64>>, side: usize) {
    let (m, n) = size(&map);
    let (m, n) = (m as i64, n as i64);
    let i_min = frontier.iter().map(|Vec2(i, _)| i).min().unwrap();
    let i_max = frontier.iter().map(|Vec2(i, _)| i).max().unwrap();
    let j_min = frontier.iter().map(|Vec2(_, j)| j).min().unwrap();
    let j_max = frontier.iter().map(|Vec2(_, j)| j).max().unwrap();
    println!("{} {}", Vec2(i_min, i_max), Vec2(j_min, j_max));
    let top = Vec2(*i_min, (j_min + j_max) / 2);
    for i in *i_min..i_max + 1 {
        if i.rem_euclid(m) == 0 {
            println!("{}", vec!["-"; (i_max - i_min) as usize + 1].join(""))
        }
        (*j_min..j_max + 1).for_each(|j| {
            if j.rem_euclid(n) == 0 {
                print!("|");
            }
            let c = if frontier.contains(&Vec2(i, j)) {
                'O'
            } else if map[i.rem_euclid(m) as usize][j.rem_euclid(n) as usize] {
                ' '
            } else {
                '#'
            };
            print!("{}", c);
        });
        print!("\n");
    }
}

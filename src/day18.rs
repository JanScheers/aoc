use crate::{pretty, size, Vec2};
use std::cmp;

pub const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

const NORTH: usize = 0b0001;
const EAST: usize = 0b0010;
const SOUTH: usize = 0b0100;
const WEST: usize = 0b1000;
const DIRS: &[Vec2<i64>] = &[
    Vec2(0, 0),
    Vec2(-1, 0),
    Vec2(0, 1),
    Vec2(0, 0),
    Vec2(1, 0),
    Vec2(0, 0),
    Vec2(0, 0),
    Vec2(0, 0),
    Vec2(0, -1),
];

pub fn parse1(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let dir = match chars.next().unwrap() {
                'U' => NORTH,
                'R' => EAST,
                'D' => SOUTH,
                'L' => WEST,
                _ => 0,
            };
            chars.next();
            let rest: String = chars.collect();
            let (length, _) = rest.split_once(" ").unwrap();
            return (dir, length.parse().unwrap());
        })
        .collect()
}

pub fn parse2(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (_, color) = line.split_once('#').unwrap();
            let dir = match color.chars().nth(5).unwrap() {
                '0' => EAST,
                '1' => SOUTH,
                '2' => WEST,
                '3' => NORTH,
                _ => 0,
            };
            let length = usize::from_str_radix(&color[..5], 16).unwrap();
            dbg!(length);
            return (dir, length);
        })
        .collect()
}

pub fn dig(commands: Vec<(usize, usize)>) -> Vec<Vec<char>> {
    let mut curr = Vec2(0, 0);
    let mut trench = vec![];
    for (dir, length) in commands.into_iter() {
        for _ in 0..length {
            curr = curr + DIRS[dir];
            trench.push((curr, dir))
        }
    }
    let top_left = trench.iter().fold(Vec2(0, 0), |a, (b, _)| {
        Vec2(cmp::min(a.0, b.0), cmp::min(a.1, b.1))
    });
    let bottom_right = trench.iter().fold(Vec2(0, 0), |a, (b, _)| {
        Vec2(cmp::max(a.0, b.0), cmp::max(a.1, b.1))
    });
    let Vec2(m, n) = bottom_right - top_left + Vec2(1, 1);

    let mut map = vec![vec!['.'; n as usize]; m as usize];
    trench = trench
        .iter()
        .map(|&(pos, dir)| {
            let abs = pos - top_left;
            map[abs.0 as usize][abs.1 as usize] = match dir {
                NORTH => '^',
                EAST => '>',
                SOUTH => 'v',
                WEST => '<',
                _ => '.',
            };
            (abs, dir)
        })
        .collect();

    pretty(&map);
    println!();
    let (m, n) = size(&map);
    let map: Vec<Vec<char>> = (0..m)
        .map(|i| {
            let mut inside = false;
            (0..n)
                .map(|j| {
                    let char = map[i][j];
                    if char == '.' {
                        return if inside { '#' } else { '.' };
                    }
                    match char {
                        '^' => inside = true,
                        'v' => inside = false,
                        '>' => {
                            let curr = trench
                                .iter()
                                .position(|(a, _)| a.0 as usize == i && a.1 as usize == j)
                                .unwrap();
                            let next = trench[(curr + 1) % trench.len()].0;
                            let diff = next - trench[curr].0;
                            if diff == DIRS[SOUTH] {
                                inside = false;
                            } else if diff == DIRS[NORTH] {
                                inside = true;
                            };
                        }
                        _ => {}
                    };
                    char
                })
                .collect()
        })
        .collect();
    pretty(&map);
    map
}

pub fn dig2(commands: Vec<(usize, usize)>) -> Vec<Vec<char>> {
    let mut curr = Vec2(0, 0);
    let mut trench = vec![];
    for (dir, length) in commands.into_iter() {
        let next = curr + length as i64 * DIRS[dir];
        if dir == SOUTH {
            trench.push((curr.1, curr.0, next.0, dir));
        } else if dir == NORTH {
            trench.push((curr.1, next.0, curr.0, dir));
        }
        curr = next;
    }
    let top_left = trench.iter().fold(Vec2(0, 0), |a, b| {
        Vec2(cmp::min(a.0, b.1), cmp::min(a.1, b.0))
    });

    trench = trench
        .iter()
        .map(|(col, a, b, dir)| (col - top_left.1, a - top_left.0, b - top_left.0, *dir))
        .collect();
    trench.sort();

    let m = trench.iter().map(|a| a.2).max().unwrap() + 1;
    let n = trench.iter().map(|a| a.0).max().unwrap() + 1;
    let map = (0..m)
        .map(|row| {
            let mut out = vec!['.'; n as usize];
            let mut cols = trench.iter().filter(|(_, a, b, _)| *a <= row && row <= *b);

            let mut start = cols.next();
            while start.is_some() {
                let (mut a, mut b) = (start.unwrap(), cols.next());
                while b.is_some() && !(a.3 == SOUTH && b.unwrap().3 == NORTH) {
                    a = b.unwrap();
                    b = cols.next();
                }
                for col in start.unwrap().0..a.0 + 1 {
                    out[col as usize] = '#';
                }
                start = b;
            }
            out
        })
        .collect();
    pretty(&map);
    return map;
}

pub fn part_one(input: &str) -> usize {
    let map = dig(parse1(input));
    map.iter()
        .map(|row| row.iter().filter(|c| **c != '.').count())
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let map1 = dig(parse1(input));
    let map2 = dig2(parse1(input));
    let (m, n) = size(&map1);
    pretty(
        &(0..m)
            .map(|row| {
                (0..n)
                    .map(|col| {
                        if map1[row][col] != '.' && map2[row][col] != '#' {
                            'X'
                        } else {
                            '.'
                        }
                    })
                    .collect()
            })
            .collect(),
    );
    println!();
    map1.iter()
        .map(|row| row.iter().filter(|c| **c != '.').count())
        .sum()
}

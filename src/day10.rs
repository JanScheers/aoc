use std::cmp;
use std::collections::HashSet;
use std::ops;

pub const INPUT1: &str = "
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

pub const INPUT2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

pub const INPUT3: &str = "..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........";

pub const INPUT4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

pub const INPUT5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point<T>(T, T);

impl<T: ops::Add<Output = T>> ops::Add<Point<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Point<T> {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub<Point<T>> for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Point<T>) -> Point<T> {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn pretty(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        let s: String = row.iter().collect();
        println!("{}", s)
    }
}

const NORTH: Point<i64> = Point(-1, 0);
const SOUTH: Point<i64> = Point(1, 0);
const WEST: Point<i64> = Point(0, -1);
const EAST: Point<i64> = Point(0, 1);

fn pipe(pos: Point<i64>, map: &Vec<Vec<char>>) -> Vec<Point<i64>> {
    let (m, n) = (map.len() as i64, map[0].len() as i64);
    let Point(i, j) = pos;
    let x = match map[i as usize][j as usize] {
        '|' => vec![NORTH, SOUTH],
        '-' => vec![EAST, WEST],
        'L' => vec![NORTH, EAST],
        'J' => vec![NORTH, WEST],
        '7' => vec![SOUTH, WEST],
        'F' => vec![SOUTH, EAST],
        _ => vec![],
    }
    .into_iter()
    .filter_map(|dir| {
        let Point(i, j) = pos + dir;
        if 0 <= i && i < m && 0 <= j && j <= n {
            Some(Point(i, j))
        } else {
            None
        }
    })
    .collect();
    return x;
}

pub fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Point<i64>>) {
    let mut map: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
    let (m, n) = (map.len() as i64, map[0].len() as i64);

    let mut start: Point<i64> = Point(-2, -2);
    'find: for i in 0..m {
        for j in 0..n {
            if map[i as usize][j as usize] == 'S' {
                start = Point(i as i64, j as i64);
                break 'find;
            }
        }
    }

    let next = [NORTH, SOUTH, EAST, WEST]
        .iter()
        .map(|dir| start + *dir)
        .find(|dir| {
            (dir.0 >= 0 && dir.0 < m)
                && (dir.1 >= 0 && dir.1 < n)
                && pipe(*dir, &map).iter().any(|rev| *rev == start)
        })
        .unwrap();

    let mut path = vec![start, next];
    loop {
        let dirs = pipe(path[path.len() - 1], &map);
        if dirs.len() == 0 {
            break;
        }
        path.push(*dirs.iter().find(|d| **d != path[path.len() - 2]).unwrap())
    }
    path.pop();
    let prev = path[path.len() - 1];
    map[start.0 as usize][start.1 as usize] = match (prev - start, next - start) {
        (NORTH, SOUTH) => '|',
        (SOUTH, NORTH) => '|',
        (EAST, WEST) => '-',
        (WEST, EAST) => '-',
        (NORTH, EAST) => 'L',
        (EAST, NORTH) => 'L',
        (NORTH, WEST) => 'J',
        (WEST, NORTH) => 'J',
        (SOUTH, WEST) => '7',
        (WEST, SOUTH) => '7',
        (SOUTH, EAST) => 'F',
        (EAST, SOUTH) => 'F',
        _ => '.',
    };
    return (map, path);
}

pub fn part_one(input: &str) -> usize {
    let (_, path) = parse(input);
    return path.len() / 2;
}

pub fn part_two(input: &str) -> usize {
    let (map, path) = parse(input);
    let path: HashSet<Point<i64>> = path.into_iter().collect();
    let (m, n) = (map.len() as i64, map[0].len() as i64);
    let cat: Vec<Vec<char>> = (0..m)
        .map(|i| {
            let mut inside = false;
            let mut from_up = false;
            let mut res = Vec::with_capacity(n as usize);

            for j in 0..n {
                if path.contains(&Point(i, j)) {
                    let c = map[i as usize][j as usize];
                    match c {
                        '|' => inside = !inside,
                        'L' => {
                            from_up = true;
                        }
                        'F' => {
                            from_up = false;
                        }
                        '7' => {
                            if from_up {
                                inside = !inside;
                            };
                        }
                        'J' => {
                            if !from_up {
                                inside = !inside;
                            }
                        }
                        _ => {}
                    }
                    res.push(c)
                } else {
                    res.push(if inside { '#' } else { ' ' });
                }
            }
            return res;
        })
        .collect();
    pretty(&cat);
    return cat
        .iter()
        .map(|row| row.iter().filter(|c| **c == '#').count())
        .sum();
}

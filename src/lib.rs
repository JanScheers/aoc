use num::Signed;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::{fmt, ops};
pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vec2<T>(T, T);

pub const NORTH: u8 = 0;
pub const EAST: u8 = 1;
pub const SOUTH: u8 = 2;
pub const WEST: u8 = 3;
pub const DIRS: &[Vec2<i64>] = &[Vec2(-1, 0), Vec2(0, 1), Vec2(1, 0), Vec2(0, -1)];

impl<T: ops::Add<Output = T>> ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.0, self.1)
    }
}

impl ops::Mul<Vec2<usize>> for usize {
    type Output = Vec2<usize>;

    fn mul(self, rhs: Vec2<usize>) -> Vec2<usize> {
        Vec2(self * rhs.0, self * rhs.1)
    }
}

impl ops::Mul<Vec2<i32>> for i32 {
    type Output = Vec2<i32>;

    fn mul(self, rhs: Vec2<i32>) -> Vec2<i32> {
        Vec2(self * rhs.0, self * rhs.1)
    }
}

impl ops::Mul<Vec2<i64>> for i64 {
    type Output = Vec2<i64>;

    fn mul(self, rhs: Vec2<i64>) -> Vec2<i64> {
        Vec2(self * rhs.0, self * rhs.1)
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub fn norm1<T: Signed>(Vec2(a, b): Vec2<T>) -> T {
    a.abs() + b.abs()
}

pub fn size<T>(mat: &Vec<Vec<T>>) -> (usize, usize) {
    (mat.len(), mat[0].len())
}

pub fn transpose<T: Copy>(mat: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let (m, n) = size(mat);
    (0..n)
        .map(|j| (0..m).map(|i| mat[i][j]).collect())
        .collect()
}

pub fn rotclck<T: Copy>(map: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let (m, n) = size(&map);
    (0..n)
        .map(|col| (0..m).rev().map(|row| map[row][col]).collect())
        .collect()
}

pub fn rotanti<T: Copy>(map: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let (m, n) = size(&map);
    (0..n)
        .rev()
        .map(|col| (0..m).map(|row| map[row][col]).collect())
        .collect()
}

pub fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };
    while b != 0 {
        (a, b) = (b, a % b);
    }
    return a;
}

pub fn lcm(v: &Vec<usize>) -> usize {
    v.iter().fold(v[0], |lcm, a| lcm / gcd(lcm, *a) * a)
}

pub fn pretty<T: ToString>(map: &Vec<Vec<T>>) {
    for row in map.iter() {
        let s: String = row.iter().map(|i| i.to_string()).collect();
        println!("{}", s)
    }
}

pub fn pretty3(map: &Vec<Vec<i32>>) {
    for row in map.iter() {
        let s: String = row.iter().map(|i| format!("{: >4}", i)).collect();
        println!("{}", s)
    }
}
pub fn pretty2(map: &Vec<&[char]>) {
    for row in map.iter() {
        let s: String = row.iter().collect();
        println!("{}", s)
    }
}

pub fn get_day(day: u32) -> String {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    let Ok(response) = client.get(url).header("Cookie", "session=53616c7465645f5f9cff0b74a418fafbe18b6fe39f2465514ac3acdc0bb808a2d43d973544cbede77ddb2dbc97ac6066766a48ae253e3db82e9f54edfd518009").send() else {
         return String::new();
    };
    return response.text().unwrap_or_else(|_| String::new());
}

//Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub struct PreOrderIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl Iterator for PreOrderIterator {
    type Item = Rc<RefCell<TreeNode>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|node| {
            if let Some(ref left) = node.borrow().left {
                self.stack.push(left.clone());
            }
            if let Some(ref right) = node.borrow().right {
                self.stack.push(right.clone());
            }
            node
        })
    }
}
pub struct InOrderIterator {
    left: Vec<Rc<RefCell<TreeNode>>>,
    right: Vec<Rc<RefCell<TreeNode>>>,
}

impl Iterator for InOrderIterator {
    type Item = Rc<RefCell<TreeNode>>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.left.pop() {
            if let Some(ref left) = node.borrow().left {
                self.left.push(left.clone());
            }
            self.right.push(node.clone());
        }
        self.right.pop().map(|node| {
            if let Some(ref right) = node.borrow().right {
                self.left.push(right.clone());
            }
            node
        })
    }
}
pub struct PostOrderIterator {
    iter: std::iter::Rev<std::vec::IntoIter<Rc<RefCell<TreeNode>>>>,
}

impl Iterator for PostOrderIterator {
    type Item = Rc<RefCell<TreeNode>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut queue = VecDeque::from([root]);
        let mut values = vec![];
        while let Some(node) = queue.pop_front() {
            values.push(node.map(|node| {
                let node = node.borrow();
                queue.push_back(node.left.clone());
                queue.push_back(node.right.clone());
                node.val
            }));
        }
        let length = values.iter().rposition(|v| v.is_some());
        values.truncate(length.unwrap_or(0) + 1);
        let values: Vec<_> = values
            .iter()
            .map(|v| v.map(|v| v.to_string()).unwrap_or(String::from("null")))
            .collect();
        format!("[{}]", values.join(","))
    }

    pub fn deserialize(data: &str) -> Option<Rc<RefCell<TreeNode>>> {
        let mut values = data[1..data.len() - 1].split(',').map(|s| {
            s.parse::<i32>()
                .ok()
                .map(|val| Rc::new(RefCell::new(TreeNode::new(val))))
        });
        let root = values.next()?;
        let (mut i, mut nodes, mut next) = (0, vec![root.clone()], vec![]);
        while let Some(node) = values.next() {
            if let Some(parent) = &nodes[i / 2] {
                if i % 2 == 0 {
                    parent.borrow_mut().left = node.clone();
                } else {
                    parent.borrow_mut().right = node.clone();
                }
            }
            node.map(|n| next.push(Some(n)));
            i += 1;
            if i / 2 >= nodes.len() {
                (i, nodes, next) = (0, next, vec![]);
            }
        }
        root
    }

    pub fn pre_order(root: Option<Rc<RefCell<TreeNode>>>, f: fn(i32)) {
        let Some(root) = root else {
            return;
        };
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            let node = node.borrow();
            f(node.val);
            if let Some(ref left) = node.left {
                stack.push(left.clone());
            }
            if let Some(ref right) = node.right {
                stack.push(right.clone());
            }
        }
    }

    pub fn in_order(root: Option<Rc<RefCell<TreeNode>>>, f: fn(i32)) {
        let Some(root) = root else {
            return;
        };
        let (mut lstack, mut rstack) = (vec![root], vec![]);
        loop {
            while let Some(node) = lstack.pop() {
                if let Some(ref left) = node.borrow().left {
                    lstack.push(left.clone());
                }
                rstack.push(node.clone());
            }
            if let Some(node) = rstack.pop() {
                let node = node.borrow();
                f(node.val);
                if let Some(ref right) = node.right {
                    lstack.push(right.clone());
                }
            } else {
                break;
            }
        }
    }

    pub fn post_order(root: Option<Rc<RefCell<TreeNode>>>, f: fn(i32)) {
        let Some(root) = root else {
            return;
        };
        let (mut lstack, mut rstack) = (vec![root], vec![]);
        loop {
            while let Some(node) = lstack.pop() {
                if let Some(ref left) = node.borrow().left {
                    lstack.push(left.clone());
                }
                if let Some(ref right) = node.borrow().right {
                    lstack.push(right.clone());
                }
                rstack.push(node.clone());
            }
            if let Some(node) = rstack.pop() {
                let node = node.borrow();
                f(node.val);
            } else {
                break;
            }
        }
    }

    pub fn pre_iter(&self) -> PreOrderIterator {
        PreOrderIterator {
            stack: vec![Rc::new(RefCell::new(TreeNode {
                val: self.val,
                left: self.left.clone(),
                right: self.right.clone(),
            }))],
        }
    }

    pub fn in_iter(&self) -> InOrderIterator {
        InOrderIterator {
            left: vec![Rc::new(RefCell::new(TreeNode {
                val: self.val,
                left: self.left.clone(),
                right: self.right.clone(),
            }))],
            right: vec![],
        }
    }

    pub fn post_iter(&self) -> PostOrderIterator {
        let mut stack = vec![Rc::new(RefCell::new(TreeNode {
            val: self.val,
            left: self.left.clone(),
            right: self.right.clone(),
        }))];
        let mut order = vec![];
        while let Some(node) = stack.pop() {
            if let Some(ref left) = node.borrow().left {
                stack.push(left.clone());
            }
            if let Some(ref right) = node.borrow().right {
                stack.push(right.clone());
            }
            order.push(node.clone());
        }
        PostOrderIterator {
            iter: order.into_iter().rev(),
        }
    }
}

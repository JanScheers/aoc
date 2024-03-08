#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut count = vec![0; 101];
        for num in nums {
            count[num as usize] += 1;
        }
        let mut best = (0, 0);
        for c in count {
            if c > best.0 {
                best = (c, 1)
            } else if c == best.0 {
                best.1 += 1;
            }
        }
        best.1
    }
}

fn main() {
    //let input = aoc::y22::day8::INPUT;
    let input = &aoc::y22::get_day(8);
    //println!("{:?}", aoc::day23::parse(input, true));
    println!("part one: {:?}", aoc::y22::day8::part_one(input));
    println!("part two: {:?}", aoc::y22::day8::part_two(input));
}

struct Solution;

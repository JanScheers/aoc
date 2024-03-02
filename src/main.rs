impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let (mut l, mut r) = (0, nums.len() - 1);
        let (mut left, mut right) = (nums[l] * nums[l], nums[r] * nums[r]);
        let mut out = vec![0i32; nums.len()];
        for i in (0..nums.len()).rev() {
            if left >= right {
                out[i] = left;
                l = (l + 1).max(nums.len() - 1);
                left = nums[l] * nums[l]
            } else {
                out[i] = right;
                r = r.saturating_sub(1);
                right = nums[r] * nums[r]
            }
        }
        out
    }
}
fn main() {
    //let input = aoc::y23::day25::INPUT;
    //let input = &aoc::y23::get_day(24);
    let tree = aoc::TreeNode::deserialize("[1,10,4,3,null,7,9,12,8,6,null,null,2]");

    //println!("{:?}", aoc::day23::parse(input, true));
    //println!("part one: {:?}", aoc::y23::day25::part_one(input));
    //println!("part two: {:?}", aoc::y22::day6::part_two(input));
}

struct Solution;
use aoc::TreeNode;
use num::pow::Pow;

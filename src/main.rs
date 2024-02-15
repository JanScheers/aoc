struct Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();
        let mut csum: i64 = nums.iter().map(|n| *n as i64).sum();
        nums.into_iter()
            .skip(2)
            .rev()
            .find_map(|num| {
                let num: i64 = num.into();
                if csum > 2 * num {
                    Some(csum)
                } else {
                    csum -= num;
                    None
                }
            })
            .unwrap_or(-1)
    }
}
fn main() {
    //println!("{:?}", Solution::sequential_digits(1, 1234567890))
}

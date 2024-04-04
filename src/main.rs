impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut depth = 0;
        s.chars()
            .map(|c| {
                match c {
                    '(' => depth += 1,
                    ')' => depth -= 1,
                    _ => (),
                };
                depth
            })
            .max()
            .unwrap()
    }
}

fn main() {
    //let input = aoc::y22::day15::INPUT;

    let input = &aoc::y22::get_day(15);
    //let v: Option<Box<ListNode>> = ListNode::serialize(&[1, 2, -3, 3, 1]);

    //println!("{:?}", aoc::y22::day15::parse(input));
    println!("part one: {:?}", aoc::y22::day15::part_one(input, 2000000));
    println!("part two: {:?}", aoc::y22::day15::part_two(input, 4000000));
}
struct Solution;

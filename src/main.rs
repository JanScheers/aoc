fn main() {
    //let input = aoc::y22::day9::INPUT;
    let input = &aoc::y22::get_day(9);
    //let v: Option<Box<ListNode>> = ListNode::serialize(&[1, 2, -3, 3, 1]);

    //println!("{:?}", aoc::day23::parse(input, true));
    println!("part one: {:?}", aoc::y22::day9::part_one(input));
    println!("part two: {:?}", aoc::y22::day9::part_two(input));
}

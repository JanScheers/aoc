fn main() {
    let input = aoc::y22::day14::INPUT;

    //let input = &aoc::y22::get_day(14);
    //let v: Option<Box<ListNode>> = ListNode::serialize(&[1, 2, -3, 3, 1]);

    //println!("{:?}", aoc::y22::day14::parse(input));
    println!("part one: {:?}", aoc::y22::day14::part_one(input));
    println!("part two: {:?}", aoc::y22::day14::part_two(input));
}

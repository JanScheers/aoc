fn main() {
    //let input = aoc::y22::day1::INPUT;
    let input = &aoc::y22::get_day(1);

    //println!("{:?}", aoc::day23::parse(input, true));
    println!("part one: {:?}", aoc::y22::day1::part_one(input));
    println!("part two: {:?}", aoc::y22::day1::part_two(input));
}

fn main() {
    //let input = aoc::y22::day6::INPUT;
    let input = &aoc::y22::get_day(6);

    //println!("{:?}", aoc::day23::parse(input, true));
    println!("part one: {:?}", aoc::y22::day6::part_one(input));
    println!("part two: {:?}", aoc::y22::day6::part_two(input));
}

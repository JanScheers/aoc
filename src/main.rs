fn main() {
    //let input = aoc::y22::day3::INPUT;
    let input = &aoc::y22::get_day(4);

    //println!("{:?}", aoc::day23::parse(input, true));
    println!("part one: {:?}", aoc::y22::day4::part_one(input));
    println!("part two: {:?}", aoc::y22::day4::part_two(input));
}

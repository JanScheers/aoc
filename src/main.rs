fn main() {
    //let input = aoc::day24::INPUT;
    let input = &aoc::get_day(24);
    //println!("{:?}", aoc::day23::parse(input, false));
    //println!("{:?}", aoc::day23::parse(input, true));
    println!(
        "part one: {:?}",
        aoc::day24::part_one(input, 200000000000000.0, 400000000000000.0)
    );
    //println!("part two: {:?}", aoc::day23::part_two(input));
}

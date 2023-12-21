use aoc::{day11, get_day};

fn main() {
    //let input = day11::INPUT;
    let input = &get_day(11);
    dbg!(day11::solve(input, 1));
    dbg!(day11::solve(input, 999_999));
    //dbg!(day10::part_two(&get_day(9)));
}

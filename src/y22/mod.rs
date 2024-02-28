pub mod day1;

const SESSION: &str = "session=53616c7465645f5f1b5e774f9590b3208d567b2e2dcd60ce52d10c6aed45725f2709551eaa39f78c528b06bb5639c7e83dd90f7080464df4314ff7c7afaa1629";

pub fn get_day(day: u32) -> String {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    client
        .get(url)
        .header("Cookie", SESSION)
        .send()
        .map(|r| r.text().unwrap_or_default())
        .unwrap_or_default()
}

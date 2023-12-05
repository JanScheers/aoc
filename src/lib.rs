pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

pub fn get_day(day: u32) -> String {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    let Ok(response) = client.get(url).header("Cookie", "session=53616c7465645f5fa452d35a782d6a769d21bdbe284e54651ec98b3d7633491f06cca83de27df1dc022bcde10715347229fff34ea48bfe0b70499b76da9bad74").send() else {
         return String::new();
    };
    return response.text().unwrap_or_else(|_| String::new());
}

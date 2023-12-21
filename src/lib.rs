use std::ops;
pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Vec2<T>(T, T);

impl<T: ops::Add<Output = T>> ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

pub fn get_day(day: u32) -> String {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    let Ok(response) = client.get(url).header("Cookie", "session=53616c7465645f5fa452d35a782d6a769d21bdbe284e54651ec98b3d7633491f06cca83de27df1dc022bcde10715347229fff34ea48bfe0b70499b76da9bad74").send() else {
         return String::new();
    };
    return response.text().unwrap_or_else(|_| String::new());
}

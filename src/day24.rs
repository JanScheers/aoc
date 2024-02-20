pub const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hail {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
}

pub fn parse(input: &str) -> Vec<Hail> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" @ ").unwrap();
            let [p, v] = [p, v]
                .map(|s| {
                    s.split(',')
                        .map(|s| s.trim().parse().unwrap())
                        .collect::<Vec<i64>>()
                })
                .to_owned();

            Hail {
                p: (p[0], p[1], p[2]),
                v: (v[0], v[1], v[2]),
            }
        })
        .collect()
}

fn solve_x(a: &Hail, b: &Hail) -> f64 {
    let (va, vb) = (a.v.1 as f64 / a.v.0 as f64, b.v.1 as f64 / b.v.0 as f64);
    (a.p.0 as f64 * va - b.p.0 as f64 * vb - a.p.1 as f64 + b.p.1 as f64) / (va - vb)
}

fn solve_y(a: &Hail, x: f64) -> f64 {
    (x - a.p.0 as f64) * a.v.1 as f64 / a.v.0 as f64 + a.p.1 as f64
}

fn solve_t(a: &Hail, x: f64) -> f64 {
    (x - a.p.0 as f64) / a.v.0 as f64
}

pub fn part_one(input: &str, low: f64, high: f64) -> i32 {
    let hail = parse(input);

    let mut count = 0;
    for i in 0..hail.len() - 1 {
        for j in i + 1..hail.len() {
            let (a, b) = (&hail[i], &hail[j]);
            let x = solve_x(a, b);
            let y = solve_y(a, x);
            if low <= x
                && x <= high
                && low <= y
                && y <= high
                && solve_t(a, x) >= 0.0
                && solve_t(b, x) >= 0.0
            {
                count += 1;
            }
        }
    }
    count
}

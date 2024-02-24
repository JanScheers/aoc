impl Solution {
    pub fn find_all_people(n: i32, mut meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        meetings.sort_unstable_by_key(|m| m[2]);
        let mut knows = vec![false; n as usize];
        knows[0] = true;
        knows[first_person as usize] = true;
        let mut left = 0;
        while left < meetings.len() {
            let time = meetings[left][2];
            let right = (left + 1..meetings.len())
                .find(|i| meetings[*i][2] != time)
                .unwrap_or(meetings.len());
            let mut groups: Vec<Vec<usize>> = vec![];
            for meet in &meetings[left..right] {
                let (a, b) = (meet[0] as usize, meet[1] as usize);
                let i = groups.iter().position(|v| v.contains(&a));
                let j = groups.iter().position(|v| v.contains(&b));
                match (i, j) {
                    (None, None) => groups.push(vec![a, b]),
                    (None, Some(j)) => groups[j].push(a),
                    (Some(i), None) => groups[i].push(b),
                    (Some(i), Some(j)) => {
                        groups[i] = [groups[i].clone(), groups[j].clone()].concat();
                        groups.remove(j);
                    }
                }
                println!("{:?}", meet);
                println!("{:?}", groups);
            }
            println!("{:?}", groups);
            for group in groups {
                if group.iter().any(|p| knows[*p]) {
                    group.into_iter().for_each(|p| knows[p] = true);
                }
            }
            left = right
        }
        knows
            .into_iter()
            .enumerate()
            .filter_map(|(p, k)| if k { Some(p as i32) } else { None })
            .collect()
    }
}

fn main() {
    //let input = aoc::day24::INPUT;
    //let input = &aoc::get_day(24);
    let meetings = [[1, 2, 2], [2, 1, 2], [2, 4, 2], [0, 3, 2]]
        .map(|v| v.to_vec())
        .to_vec();

    println!("{:?}", Solution::find_all_people(5, meetings, 4));
    //println!("{:?}", aoc::day23::parse(input, true));
    // println!("part one: {:?}", aoc::day24::part_one(input, 2e14, 4e14));
    //println!("part two: {:?}", aoc::day24::part_two(input));
}

struct Solution;

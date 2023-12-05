pub const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

pub fn parse(input: &str) -> (Vec<usize>, Vec<Vec<(usize, usize, usize)>>) {
    let mut maps = input.trim().split("\n\n");

    let Some((_, seeds)) = maps.next().and_then(|s| s.split_once(": ")) else {
        panic!("no seeds")
    };
    let seeds: Vec<usize> = seeds
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let maps: Vec<_> = maps
        .map(|map| {
            let mut lines = map.lines();
            lines.next();
            return lines
                .map(|line| {
                    let [a, b, c] = line
                        .split_whitespace()
                        .map(|i| i.parse().unwrap())
                        .collect::<Vec<usize>>()[..]
                    else {
                        panic!();
                    };
                    return (a, b, c);
                })
                .collect::<Vec<_>>();
        })
        .collect();
    return (seeds, maps);
}

fn transform(&seed: &usize, map: &Vec<(usize, usize, usize)>) -> usize {
    for &(dst, src, len) in map {
        if src <= seed && seed < src + len {
            return dst + seed - src;
        }
    }
    return seed;
}

pub fn part_one(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    let seeds = maps.iter().fold(seeds, |seeds, map| {
        seeds.iter().map(|s| transform(s, map)).collect()
    });
    dbg!(&seeds);
    return seeds.into_iter().min().unwrap();
}

pub fn part_two(input: &str) -> usize {
    let (seeds, maps) = parse(input);
    let mut acc: Vec<usize> = vec![];
    for i in 0..seeds.len() / 2 {
        let (s, l) = (seeds[i * 2], seeds[i * 2 + 1]);
        let mut rg: Vec<usize> = (s..s + l).collect();
        acc.append(&mut rg);
    }
    let seeds = acc;
    let seeds = maps.iter().fold(seeds, |seeds, map| {
        seeds.iter().map(|s| transform(s, map)).collect()
    });
    return seeds.into_iter().min().unwrap();
}

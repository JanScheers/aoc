use std::iter;
pub const INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

pub const INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum GateKind {
    Broadcast,
    FlipFlop,
    Inverter,
    Output,
}

fn parse(input: &str) -> (usize, usize, Vec<Gate>, Vec<String>) {
    let gates: Vec<(String, GateKind, Vec<String>)> = input
        .trim()
        .lines()
        .map(|line| {
            let (label, output) = line.split_once(" -> ").unwrap();
            let (label, kind) = match label.as_bytes()[0] as char {
                '%' => (&label[1..], GateKind::FlipFlop),
                '&' => (&label[1..], GateKind::Inverter),
                _ => (label, GateKind::Broadcast),
            };
            let output = output.split(", ").map(|s| s.to_owned()).collect();
            (label.to_owned(), kind, output)
        })
        .collect();
    let all_gates: Vec<_> = gates.iter().flat_map(|g| g.2.iter()).collect();
    let receive = all_gates
        .iter()
        .find(|g| !gates.iter().any(|(label, _, _)| ***g == *label))
        .unwrap();
    let receive = (receive.to_owned().to_owned(), GateKind::Output, vec![]);

    let broadcast = gates.iter().filter(|g| g.1 == GateKind::Broadcast);
    let flipflops = gates.iter().filter(|g| g.1 == GateKind::FlipFlop);
    let inverter = gates.iter().filter(|g| g.1 == GateKind::Inverter);
    let m = flipflops.clone().count();
    let n = inverter.clone().count();

    let mut gates: Vec<_> = broadcast
        .chain(flipflops)
        .chain(inverter)
        .enumerate()
        .collect();
    gates.push((gates.len(), &receive));

    let outputs: Vec<Vec<usize>> = gates
        .iter()
        .map(|(_, (_, _, output))| {
            output
                .iter()
                .map(|s| {
                    gates
                        .iter()
                        .find(|(_, (label, _, _))| label == s)
                        .unwrap()
                        .0
                })
                .collect()
        })
        .collect();
    let masks: Vec<usize> = (0..gates.len())
        .map(|g| {
            let mut mask = 0;
            for i in outputs.iter().enumerate().filter_map(|(j, out)| {
                if out.iter().any(|o| *o == g) {
                    Some(j)
                } else {
                    None
                }
            }) {
                mask += 1 << i;
            }
            return mask;
        })
        .collect();

    (
        m,
        n,
        iter::zip(masks.into_iter(), outputs.into_iter()).collect(),
        gates
            .iter()
            .map(|(_, (label, _, _))| label.to_owned())
            .collect(),
    )
}

type Gate = (usize, Vec<usize>);

pub fn part_one(input: &str) -> usize {
    let (m, n, gates, strings) = parse(input);
    dbg!(strings, gates.len());

    let mut memory = 0;
    let (mut nlow, mut nhigh) = (0, 0);
    for _ in 0..1 {
        let (mut next, mut last, mut pulses) = (0, 1, vec![(0, 0); 1 << 32]);
        while next != last {
            pret(&memory, gates.len());
            let (dst, high) = pulses[next];
            if high > 0 {
                nhigh += 1;
            } else {
                nlow += 1;
            }
            next = (next + 1) % (1 << 32);
            let (mask, output) = &gates[dst];
            let (mut bit, mut out) = (0, &vec![]);
            //print!("-{}-> {} ", high, strings[dst]);
            //pret(&memory, gates.len());
            if dst == 0 {
                bit = high;
                out = output;
            } else if dst <= n {
                if high == 0 {
                    memory ^= 1 << dst;
                    bit = ((memory & 1 << dst) > 0) as usize;
                    out = output;
                }
            } else {
                bit = ((memory & mask) != *mask) as usize;
                memory &= !(1 << dst) | (bit << dst);
                out = output;
            }

            for o in out.iter() {
                pulses[last] = (*o, bit);
                last = (last + 1) % (1 << 16);
            }
        }
    }
    nlow * nhigh
}

fn pret(memory: &usize, n: usize) {
    for i in 0..n {
        if (memory & 1 << i) > 0 {
            print!("#");
        } else {
            print!(" ");
        }
    }
    println!();
}

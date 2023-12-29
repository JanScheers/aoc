pub const INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

pub const INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
output ->  un";

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum GateKind {
    Broadcast,
    FlipFlop,
    Inverter,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct Gate {
    kind: GateKind,
    label: String,
    input: Vec<usize>,
    output: Vec<usize>,
    mem: Vec<bool>,
}

impl Gate {
    fn new(kind: GateKind, label: String) -> Self {
        Gate {
            kind,
            label,
            input: vec![],
            output: vec![],
            mem: if kind == GateKind::FlipFlop {
                vec![false]
            } else {
                vec![]
            },
        }
    }
}

fn parse(input: &str) -> Vec<Gate> {
    let gates: Vec<(String, Gate, Vec<String>)> = input
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
            (label.to_owned(), Gate::new(kind, label.to_owned()), output)
        })
        .collect();
    gates
        .iter()
        .map(|(label, gate, output)| {
            let mut gate = Gate {
                input: gates
                    .iter()
                    .enumerate()
                    .filter_map(|(j, (_, _, output))| {
                        if output.contains(label) {
                            Some(j)
                        } else {
                            None
                        }
                    })
                    .collect(),
                output: gates
                    .iter()
                    .enumerate()
                    .filter_map(|(j, (label, _, _))| {
                        if output.contains(label) {
                            Some(j)
                        } else {
                            None
                        }
                    })
                    .collect(),
                ..gate.clone()
            };
            if gate.kind == GateKind::Inverter {
                gate.mem = vec![true; gates.len()];
                for &i in gate.input.iter() {
                    gate.mem[i] = false;
                }
            }
            gate
        })
        .collect()
}

fn push(gates: &mut Vec<Gate>, start: usize) {
    let mut pulses = vec![(start, start, false)];
    while let Some((prev, curr, high)) = pulses.pop() {
        /*println!(
            "{} -{}-> {}",
            &gates[prev].label,
            if high { "high" } else { "low" },
            &gates[curr].label
        );*/
        let gate = &mut gates[curr];
        let empty: Vec<usize> = vec![];
        let (out, sig) = match gate.kind {
            GateKind::Broadcast => (gate.output.iter(), high),
            GateKind::FlipFlop => {
                if high {
                    (empty.iter(), high)
                } else {
                    gate.mem[0] = !gate.mem[0];
                    (gate.output.iter(), gate.mem[0])
                }
            }
            GateKind::Inverter => {
                gate.mem[prev] = high;
                (gate.output.iter(), !gate.mem.iter().all(|&h| h))
            }
        };
        for &next in out {
            pulses.insert(0, (curr, next, sig));
        }
    }
}

fn prstate(gates: &Vec<Gate>) {
    println!(
        "{}",
        gates
            .iter()
            .filter(|g| g.kind == GateKind::FlipFlop)
            .map(|g| if g.mem[0] { '#' } else { '.' })
            .collect::<String>()
    );
}

pub fn part_one(input: &str) {
    let mut gates = parse(input);
    let start = gates
        .iter()
        .position(|g| g.kind == GateKind::Broadcast)
        .unwrap();
    push(&mut gates, start);
    prstate(&gates);
    push(&mut gates, start);
    prstate(&gates);
    push(&mut gates, start);
    prstate(&gates);
    push(&mut gates, start);
    prstate(&gates);
    push(&mut gates, start);
    prstate(&gates);
}

use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn invert(self) -> Self {
        use Pulse::*;
        match self {
            High => Low,
            Low => High,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Module {
    FlipFlop {
        state: Pulse,
        outputs: Vec<String>,
    },
    Conjunction {
        inputs: BTreeMap<String, Pulse>,
        outputs: Vec<String>,
    },
    Broadcast {
        outputs: Vec<String>,
    },
    Dud,
}

impl Module {
    fn receive_pulse(&mut self, input: &str, pulse: Pulse) -> Vec<(String, Pulse)> {
        match self {
            Module::FlipFlop { state, outputs } => match pulse {
                Pulse::High => vec![],
                Pulse::Low => {
                    *state = state.invert();
                    outputs.iter().map(|o| (o.clone(), *state)).collect()
                }
            },
            Module::Conjunction { inputs, outputs } => {
                *inputs
                    .get_mut(input)
                    .expect("Didn't expect input from {input}") = pulse;
                let output = if inputs.values().all(|v| *v == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                outputs.iter().map(|o| (o.clone(), output)).collect()
            }
            Module::Broadcast { outputs } => outputs.iter().map(|o| (o.clone(), pulse)).collect(),
            Module::Dud => vec![],
        }
    }

    fn parse(input: &str) -> (String, Module) {
        let mut split = input.split(" -> ");
        let left = split.next().unwrap();
        let right = split.next().unwrap();
        assert!(split.next().is_none());

        let outputs = right.split(",").map(|o| o.trim().to_string()).collect();

        match &left[0..1] {
            "%" => (
                left[1..].to_string(),
                Module::FlipFlop {
                    state: Pulse::Low,
                    outputs,
                },
            ),
            "&" => (
                left[1..].to_string(),
                Module::Conjunction {
                    inputs: BTreeMap::new(),
                    outputs,
                },
            ),
            _ if left == "broadcaster" => (left.to_string(), Module::Broadcast { outputs }),
            _ => (left.to_string(), Module::Dud),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Puzzle {
    modules: HashMap<String, Module>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut modules: HashMap<String, Module> = input.lines().map(Module::parse).collect();

        let mut duds_to_insert = vec![];
        let mut conjunction_inputs: HashMap<String, Vec<String>> = HashMap::new();

        // Now we need to figure out the inputs for each Conjunction module
        for (mod_name, module) in modules.iter() {
            let outputs = match module {
                Module::FlipFlop { state: _, outputs } => outputs,
                Module::Conjunction { inputs: _, outputs } => outputs,
                Module::Broadcast { outputs } => outputs,
                Module::Dud => continue,
            };

            for output in outputs {
                if let Some(target_mod) = modules.get(output) {
                    if let Module::Conjunction { .. } = target_mod {
                        conjunction_inputs
                            .entry(output.clone())
                            .and_modify(|v| v.push(mod_name.clone()))
                            .or_insert_with(|| vec![mod_name.clone()]);
                    }
                } else {
                    duds_to_insert.push(output.clone());
                }
            }
        }

        for (module, to_insert) in conjunction_inputs {
            let Some(Module::Conjunction { inputs, outputs: _ }) = modules.get_mut(&module) else {
                panic!("Module name in map isn't a conjunction")
            };

            for input in to_insert {
                inputs.insert(input, Pulse::Low);
            }
        }

        for dud in duds_to_insert {
            modules.insert(dud, Module::Dud);
        }

        Ok(Self { modules })
    }
}

#[test]
fn test_puzzle_parse() {
    let puzzle: Puzzle = TEST_STR1.parse().unwrap();
    assert_eq!(
        puzzle,
        Puzzle {
            modules: HashMap::from([
                (
                    "broadcaster".to_string(),
                    Module::Broadcast {
                        outputs: vec!["a".to_string(), "b".to_string(), "c".to_string()]
                    }
                ),
                (
                    "a".to_string(),
                    Module::FlipFlop {
                        state: Pulse::Low,
                        outputs: vec!["b".to_string()]
                    }
                ),
                (
                    "b".to_string(),
                    Module::FlipFlop {
                        state: Pulse::Low,
                        outputs: vec!["c".to_string()]
                    }
                ),
                (
                    "c".to_string(),
                    Module::FlipFlop {
                        state: Pulse::Low,
                        outputs: vec!["inv".to_string()]
                    }
                ),
                (
                    "inv".to_string(),
                    Module::Conjunction {
                        inputs: BTreeMap::from([("c".to_string(), Pulse::Low)]),
                        outputs: vec!["a".to_string()]
                    }
                ),
            ]),
        }
    );
}

struct PulseCount {
    high: u64,
    low: u64,
}

struct DirectedPulse {
    from: String,
    to: String,
    pulse: Pulse,
}

impl Puzzle {
    fn push_button(&mut self, iteration: u64) -> (PulseCount, bool) {
        let mut pulses = VecDeque::new();
        let mut sent_low_to_rx = false;
        pulses.push_back(DirectedPulse {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            pulse: Pulse::Low,
        });
        let mut counts = PulseCount {
            low: 1, // from button
            high: 0,
        };

        while let Some(DirectedPulse { from, to, pulse }) = pulses.pop_front() {
            if pulse == Pulse::High && to == "kc" {
                println!("kc received high pulse from {from} at iteration {iteration}");
            }

            let module = self
                .modules
                .get_mut(&to)
                .unwrap_or_else(|| panic!("No module with name {to}"));
            let resulting_pulses = module.receive_pulse(&from, pulse);
            for (dest, pulse) in resulting_pulses {
                match pulse {
                    Pulse::High => counts.high += 1,
                    Pulse::Low => counts.low += 1,
                }
                if dest == "rx" && pulse == Pulse::Low {
                    sent_low_to_rx = true;
                }
                pulses.push_back(DirectedPulse {
                    from: to.clone(),
                    to: dest,
                    pulse: pulse,
                });
            }
        }

        (counts, sent_low_to_rx)
    }
}

fn part1(input: &str) -> u64 {
    let mut puzzle: Puzzle = input.parse().unwrap();
    let mut counts = PulseCount { high: 0, low: 0 };
    for i in 0..1000 {
        let this_counts = puzzle.push_button(i).0;
        counts.high += this_counts.high;
        counts.low += this_counts.low;
    }
    counts.high * counts.low
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_STR1), 32000000);
    assert_eq!(part1(TEST_STR2), 11687500);
}

fn part2(input: &str) -> u64 {
    let mut puzzle: Puzzle = input.parse().unwrap();
    for i in 1.. {
        if i % 100000 == 0 {
            println!("On iteration {i}");
        }
        if puzzle.push_button(i).1 {
            return i;
        }
    }
    unreachable!()
}

fn main() {
    let input = &std::fs::read_to_string("input.txt").expect("input.txt should exist");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

const TEST_STR1: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

const TEST_STR2: &str = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

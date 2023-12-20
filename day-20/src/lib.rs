use std::collections::HashMap;
use std::fmt::Display;
use std::fs;

use nom::character::complete::{alpha1, line_ending};
use nom::{branch::alt, bytes::complete::tag, multi::separated_list1, IResult};
use petgraph::dot::Dot;
use petgraph::graph::DiGraph;

pub fn process_part_1(input: &str) -> String {
    let (input, parsed_modules) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let modules = make_modules(parsed_modules);
    let mut modules = modules;
    let mut high_pulses: u64 = 0;
    let mut low_pulses: u64 = 0;
    for _ in 0..1000 {
        let mut pulses = vec![Pulse {
            source: "button".to_string(),
            target: "broadcaster".to_string(),
            pulse_type: PulseType::Low,
        }];

        while let Some(pulse) = pulses.pop() {
            match pulse.pulse_type {
                PulseType::High => high_pulses += 1,
                PulseType::Low => low_pulses += 1,
            }
            let target_module = modules.get_mut(&pulse.target).unwrap();
            pulses.extend(target_module.process(&pulse.source, pulse.pulse_type));
        }
    }
    (high_pulses * low_pulses).to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (input, parsed_modules) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let graph = make_graph(&parsed_modules);
    let dot = Dot::with_attr_getters(
        &graph,
        &[],
        &|_, _| String::new(),
        &|_, (_, node)| match node.module_type {
            ModuleType::FlipFlop => "shape=diamond".to_string(),
            ModuleType::Conjunction => "shape=oval".to_string(),
            ModuleType::Broadcast => "shape=house".to_string(),
            ModuleType::Output => "shape=box".to_string(),
        },
    );
    fs::write("./graph.dot", format!("{}", dot)).unwrap();
    "Look at the graph in graph.dot. \
    In this example there are four binary counters with a conjuction node in the middle. \
    Some parts of the binary counter nodes have an output toward the middle conjuction (these are the 1-s), others don't (these are the zeroes). \
    Figure out the four binary numbers from the counters. (When all the counter parts, that have a connection to the center conjunction are on, that is when the conjunction triggers) \
    Once you have the four binary numbers, find their least common multiple, that is the answer".to_string()
}

fn make_graph<'a>(parsed_modules: &[ParsedModule<'a>]) -> DiGraph<Node<'a>, String> {
    let mut graph = DiGraph::new();
    parsed_modules.iter().for_each(|module| {
        graph.add_node(Node {
            name: module.name,
            module_type: module.module_type,
        });
    });
    parsed_modules.iter().for_each(|module| {
        let a = graph
            .node_indices()
            .find(|i| graph[*i].name == module.name)
            .unwrap();
        module.output.iter().for_each(|output| {
            let b = graph
                .node_indices()
                .find(|i| graph[*i].name == *output)
                .unwrap();
            graph.add_edge(a, b, "".to_string());
        });
    });
    graph
}

fn parse_input(input: &str) -> IResult<&str, Vec<ParsedModule<'_>>> {
    let (input, parsed_modules) = separated_list1(line_ending, parse_module)(input)?;
    let mut parsed_modules = parsed_modules;
    let mut outputs: HashMap<&str, Vec<&str>> = HashMap::new();
    parsed_modules.iter().for_each(|parsed_module| {
        parsed_module.output.iter().for_each(|output| {
            outputs
                .entry(output)
                .and_modify(|v| v.push(parsed_module.name))
                .or_default();
        })
    });
    outputs.iter().for_each(|(output, _)| {
        let existing_parsed_module = parsed_modules
            .iter()
            .find(|parsed_module| parsed_module.name == *output);
        if existing_parsed_module.is_none() {
            parsed_modules.push(ParsedModule {
                module_type: ModuleType::Output,
                name: output,
                output: vec![],
            });
        }
    });
    Ok((input, parsed_modules))
}

fn parse_module(input: &str) -> IResult<&str, ParsedModule<'_>> {
    alt((
        parse_broadcast_module,
        parse_flip_flop_module,
        parse_conjunction_module,
    ))(input)
}

fn parse_broadcast_module(input: &str) -> IResult<&str, ParsedModule<'_>> {
    let (input, name) = tag("broadcaster")(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, output) = parse_outputs(input)?;
    let module = ParsedModule {
        module_type: ModuleType::Broadcast,
        name,
        output,
    };
    Ok((input, module))
}

fn parse_flip_flop_module(input: &str) -> IResult<&str, ParsedModule<'_>> {
    let (input, _) = tag("%")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, output) = parse_outputs(input)?;
    let module = ParsedModule {
        module_type: ModuleType::FlipFlop,
        name,
        output,
    };
    Ok((input, module))
}

fn parse_conjunction_module(input: &str) -> IResult<&str, ParsedModule<'_>> {
    let (input, _) = tag("&")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, output) = parse_outputs(input)?;
    let module = ParsedModule {
        module_type: ModuleType::Conjunction,
        name,
        output,
    };
    Ok((input, module))
}

fn parse_outputs(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(input)
}

fn make_modules(parsed_modules: Vec<ParsedModule>) -> HashMap<String, Box<dyn Module>> {
    parsed_modules
        .iter()
        .map(|parsed_module| {
            let name = parsed_module.name.to_string();
            let input = parsed_modules
                .iter()
                .filter_map(|other_module| {
                    other_module.output.iter().find_map(|&output| {
                        if output == parsed_module.name {
                            Some(other_module.name.to_string())
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>();
            let output = parsed_module.output.iter().map(|o| o.to_string()).collect();
            let module: Box<dyn Module> = match parsed_module.module_type {
                ModuleType::FlipFlop => Box::new(FlipFlopModule {
                    name,
                    output,
                    state: false,
                }),
                ModuleType::Conjunction => Box::new(ConjunctionModule {
                    latest_inputs: input
                        .iter()
                        .map(|input| (input.clone(), PulseType::Low))
                        .collect(),
                    name,
                    output,
                }),
                ModuleType::Broadcast => Box::new(BroadcastModule { name, output }),
                ModuleType::Output => Box::new(OutputModule {}),
            };
            (parsed_module.name.to_string(), module)
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
    Output,
}

struct ParsedModule<'a> {
    module_type: ModuleType,
    name: &'a str,
    output: Vec<&'a str>,
}

#[derive(Debug)]
struct FlipFlopModule {
    name: String,
    output: Vec<String>,
    state: bool,
}

#[derive(Debug)]
struct ConjunctionModule {
    name: String,
    output: Vec<String>,
    latest_inputs: HashMap<String, PulseType>,
}

#[derive(Debug)]
struct BroadcastModule {
    name: String,
    output: Vec<String>,
}

#[derive(Debug)]
struct OutputModule {}

#[derive(Debug, Clone, Copy)]
enum PulseType {
    High,
    Low,
}

struct Pulse {
    source: String,
    target: String,
    pulse_type: PulseType,
}

trait Module {
    fn process(&mut self, input_source: &str, input: PulseType) -> Vec<Pulse>;
}

impl Module for FlipFlopModule {
    fn process(&mut self, _input_source: &str, input: PulseType) -> Vec<Pulse> {
        match input {
            PulseType::High => vec![],
            PulseType::Low => {
                self.state = !self.state;
                self.output
                    .iter()
                    .map(|output| Pulse {
                        source: self.name.clone(),
                        target: output.clone(),
                        pulse_type: if self.state {
                            PulseType::High
                        } else {
                            PulseType::Low
                        },
                    })
                    .collect()
            }
        }
    }
}

impl Module for ConjunctionModule {
    fn process(&mut self, input_source: &str, input: PulseType) -> Vec<Pulse> {
        self.latest_inputs.insert(input_source.to_string(), input);
        let output_pulse = if self
            .latest_inputs
            .values()
            .all(|pulse| matches!(pulse, PulseType::High))
        {
            PulseType::Low
        } else {
            PulseType::High
        };
        self.output
            .iter()
            .map(|output| Pulse {
                source: self.name.clone(),
                target: output.clone(),
                pulse_type: output_pulse,
            })
            .collect()
    }
}

impl Module for BroadcastModule {
    fn process(&mut self, _input_source: &str, input: PulseType) -> Vec<Pulse> {
        self.output
            .iter()
            .map(|output| Pulse {
                source: self.name.clone(),
                target: output.clone(),
                pulse_type: input,
            })
            .collect()
    }
}

impl Module for OutputModule {
    fn process(&mut self, _input_source: &str, _input: PulseType) -> Vec<Pulse> {
        vec![]
    }
}

struct Node<'a> {
    name: &'a str,
    module_type: ModuleType,
}

impl Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT_1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const INPUT_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[rstest]
    #[case(INPUT_1, "32000000")]
    #[case(INPUT_2, "11687500")]
    fn part_1_works(#[case] input: &str, #[case] expected: &str) {
        let result = process_part_1(input);
        assert_eq!(result, expected);
    }
}

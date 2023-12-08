use std::collections::BTreeMap;

use num::integer;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

pub fn process_part_1(input: &str) -> String {
    let (_, (instructions, nodes)) = parse_input(input).unwrap();
    let nodes = nodes
        .into_iter()
        .map(|node| (node.label, node))
        .collect::<BTreeMap<_, _>>();
    let mut node = "AAA";
    let mut steps = 0;
    for instruction in instructions.iter().cycle() {
        steps += 1;
        node = nodes[node].next(&nodes, instruction).label;
        if node == "ZZZ" {
            break;
        }
    }
    steps.to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (_, (instructions, nodes)) = parse_input(input).unwrap();
    let nodes = nodes
        .into_iter()
        .map(|node| (node.label, node))
        .collect::<BTreeMap<_, _>>();
    let starting_nodes = nodes
        .values()
        .filter(|node| node.label.ends_with('A'))
        .collect::<Vec<_>>();
    starting_nodes
        .iter()
        .map(|node| {
            let mut steps = 0_u64;
            let mut node = *node;
            for instruction in instructions.iter().cycle() {
                if node.label.ends_with('Z') {
                    break;
                }
                steps += 1;
                node = node.next(&nodes, instruction);
            }
            steps
        })
        .reduce(integer::lcm)
        .unwrap()
        .to_string()
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Instruction>, Vec<Node<'_>>)> {
    let (input, instructions) = many1(alt((
        tag("L").map(|_| Instruction::Left),
        tag("R").map(|_| Instruction::Right),
    )))(input)?;
    let (input, _) = many0(line_ending)(input)?;
    let (input, nodes) = separated_list1(line_ending, parse_node)(input)?;
    Ok((input, (instructions, nodes)))
}

fn parse_node(input: &str) -> IResult<&str, Node<'_>> {
    let (input, (label, (left, right))) = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    )(input)?;
    Ok((input, Node { label, left, right }))
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Hash)]
struct Node<'a> {
    label: &'a str,
    left: &'a str,
    right: &'a str,
}

impl Node<'_> {
    fn next<'b>(
        &self,
        nodes: &'b BTreeMap<&str, Node<'_>>,
        instruction: &Instruction,
    ) -> &Node<'b> {
        let next_label = match instruction {
            Instruction::Left => self.left,
            Instruction::Right => self.right,
        };
        &nodes[next_label]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT_1);
        assert_eq!(result, "6");
    }

    const INPUT_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT_2);
        assert_eq!(result, "6");
    }
}

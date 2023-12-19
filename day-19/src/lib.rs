use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::ops::{Add, RangeInclusive};

use nom::character;
use nom::character::complete::{alpha1, line_ending};
use nom::multi::many1;
use nom::{
    branch::alt, bytes::complete::tag, multi::separated_list1, sequence::delimited, IResult, Parser,
};
use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use petgraph::stable_graph::NodeIndex;
use petgraph::Graph;

pub fn process_part_1(input: &str) -> String {
    let (input, (workflows, parts)) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    parts
        .iter()
        .filter(|part| {
            let mut workflow = workflows.get("in").unwrap();
            let mut rules = workflow.rules.iter();
            let mut rule = rules.next().unwrap();
            let accepted = loop {
                match rule {
                    Rule::Jump { target } => {
                        if target == &"R" {
                            break false;
                        }
                        if target == &"A" {
                            break true;
                        }
                        workflow = workflows.get(target).unwrap();
                        rules = workflow.rules.iter();
                        rule = rules.next().unwrap();
                    }
                    Rule::ConditionalJump { condition, target } => {
                        if condition.passes(part) {
                            if target == &"R" {
                                break false;
                            }
                            if target == &"A" {
                                break true;
                            }
                            workflow = workflows.get(target).unwrap();
                            rules = workflow.rules.iter();
                            rule = rules.next().unwrap();
                        } else if let Some(next_rule) = rules.next() {
                            rule = next_rule;
                        } else {
                            break false;
                        }
                    }
                }
            };
            accepted
        })
        .map(|part| part.ratings.values().sum::<i64>())
        .sum::<i64>()
        .to_string()
}

pub fn process_part_2(input: &str) -> String {
    let (input, (workflows, _)) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let graph = make_graph(&workflows);
    fs::write("./graph.dot", format!("{}", Dot::with_config(&graph, &[]))).unwrap();
    let root = graph.node_indices().find(|i| graph[*i] == "in").unwrap();
    let paths = walk(&graph, root, Conditions(vec![]));
    paths
        .iter()
        .map(|path| {
            let mut ranges: HashMap<&str, RangeInclusive<i64>> = HashMap::new();
            ranges.insert("x", 1..=4000);
            ranges.insert("m", 1..=4000);
            ranges.insert("a", 1..=4000);
            ranges.insert("s", 1..=4000);
            for condition in &path.0 {
                ranges.entry(condition.part_category).and_modify(|range| {
                    match condition.relation {
                        Relation::Lt => {
                            if condition.value - 1 < *range.end() {
                                *range = *range.start()..=(condition.value - 1);
                            }
                        }
                        Relation::Gt => {
                            if condition.value + 1 > *range.start() {
                                *range = (condition.value + 1)..=*range.end();
                            }
                        }
                    }
                });
            }
            ranges
                .values()
                .map(|r| r.end() - r.start() + 1)
                .product::<i64>()
        })
        .sum::<i64>()
        .to_string()
}

fn walk<'a>(
    graph: &'a Graph<&str, Conditions<'a>>,
    node_index: NodeIndex,
    conditions: Conditions<'a>,
) -> Vec<Conditions<'a>> {
    let node = graph.node_weight(node_index).unwrap();
    if *node == "A" {
        return vec![conditions];
    }
    let mut result = vec![];
    let mut walker = graph.neighbors(node_index).detach();
    while let Some((edge_index, next_node_index)) = walker.next(graph) {
        let edge = graph.edge_weight(edge_index).unwrap();
        let result_conditions = walk(graph, next_node_index, conditions.clone() + edge.clone());
        result.extend(result_conditions);
    }
    result
}

fn make_graph<'a>(workflows: &'a HashMap<&str, Workflow<'a>>) -> DiGraph<&'a str, Conditions<'a>> {
    let mut graph = DiGraph::<&str, Conditions<'_>>::new();
    for workflow_name in workflows.keys() {
        graph.add_node(workflow_name);
    }
    graph.add_node("A");
    graph.add_node("R");
    for (workflow_name, workflow) in workflows {
        let a = graph
            .node_indices()
            .find(|i| graph[*i] == *workflow_name)
            .unwrap();
        let mut conditions = vec![];
        for rule in workflow.rules.iter() {
            let (target, edge) = match rule {
                Rule::Jump { target } => (target, conditions.clone()),
                Rule::ConditionalJump { condition, target } => {
                    let mut current_conditions = conditions.clone();
                    current_conditions.push(*condition);
                    conditions.push(condition.negate());
                    (target, current_conditions)
                }
            };
            let b = graph.node_indices().find(|i| graph[*i] == *target).unwrap();
            graph.add_edge(a, b, edge.into());
        }
    }
    graph
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<&str, Workflow>, Vec<Part>)> {
    let (input, workflows) = separated_list1(line_ending, parse_workflow)(input)?;
    let (input, _) = many1(line_ending)(input)?;
    let (input, parts) = separated_list1(line_ending, parse_part)(input)?;
    let workflows = workflows.into_iter().map(|w| (w.name, w)).collect();
    Ok((input, (workflows, parts)))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = alpha1(input)?;
    let (input, rules) =
        delimited(tag("{"), separated_list1(tag(","), parse_rule), tag("}"))(input)?;
    let workflow = Workflow { name, rules };
    Ok((input, workflow))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    alt((parse_rule_conditional_jump, parse_rule_jump))(input)
}

fn parse_rule_jump(input: &str) -> IResult<&str, Rule> {
    let (input, target) = alpha1(input)?;
    let rule = Rule::Jump { target };
    Ok((input, rule))
}

fn parse_rule_conditional_jump(input: &str) -> IResult<&str, Rule> {
    let (input, part_category) = alpha1(input)?;
    let (input, relation) = alt((
        tag("<").map(|_| Relation::Lt),
        tag(">").map(|_| Relation::Gt),
    ))(input)?;
    let (input, value) = character::complete::i64(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, target) = alpha1(input)?;
    let condition = Condition {
        part_category,
        relation,
        value,
    };
    let rule = Rule::ConditionalJump { condition, target };
    Ok((input, rule))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, ratings) =
        delimited(tag("{"), separated_list1(tag(","), parse_rating), tag("}"))(input)?;
    let part = Part {
        ratings: ratings.into_iter().collect(),
    };
    Ok((input, part))
}

fn parse_rating(input: &str) -> IResult<&str, (&str, i64)> {
    let (input, category) = alpha1(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, value) = character::complete::i64(input)?;
    Ok((input, (category, value)))
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug)]
enum Rule<'a> {
    Jump {
        target: &'a str,
    },
    ConditionalJump {
        condition: Condition<'a>,
        target: &'a str,
    },
}

#[derive(Debug, Clone)]
struct Conditions<'a>(Vec<Condition<'a>>);

impl<'a> From<Vec<Condition<'a>>> for Conditions<'a> {
    fn from(conditions: Vec<Condition<'a>>) -> Self {
        Conditions(conditions)
    }
}

impl<'a> Add for Conditions<'a> {
    type Output = Conditions<'a>;

    fn add(self, rhs: Conditions<'a>) -> Self::Output {
        let mut conditions = self.0;
        conditions.extend(rhs.0);
        Conditions(conditions)
    }
}

impl Display for Conditions<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        write!(f, "]")
    }
}

#[derive(Debug, Clone, Copy)]
struct Condition<'a> {
    part_category: &'a str,
    relation: Relation,
    value: i64,
}

impl<'a> Condition<'a> {
    fn passes(&self, part: &Part) -> bool {
        let part_rating = part.ratings.get(self.part_category).unwrap();
        match self.relation {
            Relation::Lt => part_rating < &self.value,
            Relation::Gt => part_rating > &self.value,
        }
    }

    fn negate(&self) -> Condition<'a> {
        let (relation, value) = match self.relation {
            Relation::Lt => (Relation::Gt, self.value - 1),
            Relation::Gt => (Relation::Lt, self.value + 1),
        };
        Condition {
            part_category: self.part_category,
            relation,
            value,
        }
    }
}

impl Display for Condition<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.relation {
            Relation::Lt => write!(f, "{}<{}", self.part_category, self.value),
            Relation::Gt => write!(f, "{}>{}", self.part_category, self.value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Relation {
    Lt,
    Gt,
}

#[derive(Debug)]
struct Part<'a> {
    ratings: HashMap<&'a str, i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "19114");
    }

    #[test]
    fn part_2_works() {
        let result = process_part_2(INPUT);
        assert_eq!(result, "167409079868000");
    }
}

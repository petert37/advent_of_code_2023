use std::{
    collections::HashSet,
    fs::{self},
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::{dot::Dot, Graph};
use rustworkx_core::connectivity::stoer_wagner_min_cut;

pub fn process_part_1(input: &str) -> String {
    let (input, lines) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");
    let graph = make_graph(&lines);
    let dot = Dot::new(&graph);
    fs::write("./graph.dot", format!("{}", dot)).unwrap();
    let min_cut_res: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));
    if let Ok(Some((_size, cut))) = min_cut_res {
        ((graph.node_count() - cut.len()) * cut.len()).to_string()
    } else {
        "No result found".to_string()
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    separated_list1(line_ending, parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1))(input)
}

fn make_graph<'a>(
    lines: &'a [(&'a str, Vec<&'a str>)],
) -> Graph<&'a str, &'a str, petgraph::prelude::Undirected> {
    let mut nodes = HashSet::new();
    let mut edges = vec![];
    lines.iter().for_each(|(node, node_edges)| {
        nodes.insert(*node);
        node_edges.iter().for_each(|edge| {
            nodes.insert(*edge);
            edges.push((*node, *edge));
        })
    });
    let mut graph = Graph::new_undirected();
    nodes.iter().for_each(|node| {
        graph.add_node(*node);
    });
    edges.iter().for_each(|(from, to)| {
        let a = graph.node_indices().find(|i| graph[*i] == *from).unwrap();
        let b = graph.node_indices().find(|i| graph[*i] == *to).unwrap();
        graph.add_edge(a, b, "");
    });
    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn part_1_works() {
        let result = process_part_1(INPUT);
        assert_eq!(result, "54");
    }
}

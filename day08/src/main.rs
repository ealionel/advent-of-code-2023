use std::{collections::HashMap, fs};

type Graph<'a> = HashMap<String, Vec<&'a str>>;

fn parse_node_line(line: &str) -> (&str, Vec<&str>) {
    let parts = line.split(" = ").collect::<Vec<&str>>();

    let node = parts[0];
    let children: Vec<&str> = parts[1][1..parts[1].len() - 1]
        .split(", ")
        .collect::<Vec<&str>>();

    (node, children)
}

fn parse_input(input: &str) -> (&str, Graph) {
    let mut graph: Graph = HashMap::new();

    let mut line_iter = input.lines();

    let directions = line_iter.next().unwrap();
    line_iter.next();

    for line in line_iter {
        let (node, children) = parse_node_line(line);

        graph.insert(String::from(node), children);
    }
    (directions, graph)
}

fn traverse_graph(graph: &Graph, directions: &str) -> i32 {
    let mut current_node = "AAA";

    let mut steps = 0;

    for direction in directions.chars().cycle() {
        // println!(
        //     "Current node: {} -> {}{:?}",
        //     current_node, direction, current_children
        // );
        steps += 1;
        let children = graph.get(current_node).unwrap();

        match direction {
            'L' => {
                current_node = children[0];
            }
            'R' => {
                current_node = children[1];
            }
            _ => panic!("Invalid direction: {}", direction),
        }

        if current_node == "ZZZ" {
            break;
        }
    }

    steps
}

fn traverse_graph_2(graph: &Graph, directions: &str, start_node: &str) -> i32 {
    let mut current_node = start_node;

    let mut steps = 0;

    for direction in directions.chars().cycle() {
        // println!(
        //     "Current node: {} -> {}{:?}",
        //     current_node, direction, current_children
        // );
        steps += 1;
        let children = graph.get(current_node).unwrap();

        match direction {
            'L' => {
                current_node = children[0];
            }
            'R' => {
                current_node = children[1];
            }
            _ => panic!("Invalid direction: {}", direction),
        }

        if current_node.ends_with("Z") {
            break;
        }
    }

    steps
}

// fn traverse_graph_multiple(graph: &Graph, directions: &str) -> i32 {
//     let starting_nodes = find_starting_nodes(graph);

//     let mut current_nodes = starting_nodes;

//     let mut steps = 0;

//     for direction in directions.chars().cycle() {
//         // println!("Current nodes: {:?} -> {}", current_nodes, direction);
//         steps += 1;
//         let mut next_nodes: Vec<String> = Vec::new();

//         for node in current_nodes {
//             let children = graph.get(&node).unwrap();

//             match direction {
//                 'L' => {
//                     next_nodes.push(children[0].to_string());
//                 }
//                 'R' => {
//                     next_nodes.push(children[1].to_string());
//                 }
//                 _ => panic!("Invalid direction: {}", direction),
//             }
//         }

//         current_nodes = next_nodes;

//         if are_all_nodes_final(&current_nodes) {
//             println!("Found final nodes: {:?}", current_nodes);
//             break;
//         }
//     }

//     steps
// }

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm_vec(numbers: Vec<u64>) -> u64 {
    numbers.into_iter().fold(1, |a, b| lcm(a, b))
}

fn traverse_graph_multiple(graph: &Graph, directions: &str) -> u64 {
    let starting_nodes = find_starting_nodes(graph);

    let all_steps: Vec<u64> = starting_nodes
        .iter()
        .map(|node| traverse_graph_2(graph, directions, node) as u64)
        .collect();

    let steps = lcm_vec(all_steps);

    steps
}

fn find_starting_nodes(graph: &Graph) -> Vec<String> {
    let mut starting_nodes: Vec<String> = Vec::new();

    for (node, _) in graph {
        if node.ends_with("A") {
            starting_nodes.push(node.to_string());
        }
    }

    starting_nodes
}

fn are_all_nodes_final(nodes: &Vec<String>) -> bool {
    for node in nodes {
        if !node.ends_with("Z") {
            return false;
        }
    }

    true
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let (directions, graph) = parse_input(&input);

    let answer1 = traverse_graph(&graph, directions);

    let answer2 = traverse_graph_multiple(&graph, directions);

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_node_line() {
        let line = "FCG = (PLG, GXC)";
        let (node, children) = parse_node_line(line);

        assert_eq!(node, "FCG");
        assert_eq!(children, vec!["PLG", "GXC"]);
    }

    #[test]
    fn test_parse_input() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let (directions, graph) = parse_input(input);

        assert_eq!(directions, "RL");
        assert_eq!(graph.len(), 7);
        assert_eq!(*graph.get("AAA").unwrap(), vec!["BBB", "CCC"]);
        assert_eq!(*graph.get("BBB").unwrap(), vec!["DDD", "EEE"]);
        assert_eq!(*graph.get("CCC").unwrap(), vec!["ZZZ", "GGG"]);
    }

    #[test]
    fn test_traverse_graph() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let (directions, graph) = parse_input(input);

        let steps = traverse_graph(&graph, &directions);

        assert_eq!(steps, 2)
    }

    #[test]
    fn test_traverse_graph_with_cycles() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let (directions, graph) = parse_input(input);

        let steps = traverse_graph(&graph, &directions);

        assert_eq!(steps, 6)
    }

    #[test]
    fn test_find_starting_nodes() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let (_, graph) = parse_input(input);

        let starting_nodes = find_starting_nodes(&graph);

        assert_eq!(starting_nodes, vec!["22A", "11A"]);
    }

    #[test]
    fn test_traverse_graph_multiple() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let (directions, graph) = parse_input(input);

        let steps = traverse_graph_multiple(&graph, &directions);

        assert_eq!(steps, 6);
    }
}

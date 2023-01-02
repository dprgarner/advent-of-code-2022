use itertools::Itertools;
use regex::Regex;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fmt::Display,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Valve(char, char);

impl From<&str> for Valve {
    fn from(s: &str) -> Self {
        let chars = s.chars().collect_vec();
        if chars.len() != 2 {
            panic!("Valve name should be a string with two chars");
        }
        Valve(chars[0], chars[1])
    }
}

impl Display for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

fn parse_line(line: &str) -> Result<(Valve, Vec<Valve>, i32), Box<dyn Error>> {
    let re = Regex::new("Valve ([A-Z]+) has flow rate=(\\d+);").unwrap();

    let captures = re.captures(line).ok_or("No match")?;

    let valve: Valve = captures
        .get(1)
        .ok_or("Expected a valve capture group")?
        .as_str()
        .into();
    let flow = captures
        .get(2)
        .ok_or("Expected a flow capture group")?
        .as_str()
        .parse()?;

    let neighbours: Vec<Valve> = line
        .split_once("to valve")
        .ok_or("Incorrect input text format")?
        .1
        .split_once(' ')
        .ok_or("Incorrect input text format")?
        .1
        .split(", ")
        .map(Valve::from)
        .collect_vec();

    Ok((valve, neighbours, flow))
}

fn parse_input(
    input: impl Iterator<Item = String>,
) -> Result<(HashMap<Valve, i32>, HashMap<Valve, Vec<Valve>>), Box<dyn Error>> {
    let mut flows = HashMap::new();
    let mut neighbours = HashMap::new();
    for line in input {
        let (valve, valve_neighbours, flow) = parse_line(&line)?;
        flows.insert(valve, flow);
        neighbours.insert(valve, valve_neighbours);
    }

    Ok((flows, neighbours))
}

/// Calculates the minimum distances between non-trivial nodes.
fn build_distances(
    non_trivial_nodes: &HashSet<Valve>,
    neighbours: &HashMap<Valve, Vec<Valve>>,
) -> HashMap<(Valve, Valve), i32> {
    let mut distances = HashMap::new();

    for start in non_trivial_nodes {
        let mut locations_to_try = VecDeque::new();
        locations_to_try.push_back(start);
        let mut distance_to_start_map: HashMap<&Valve, _> = HashMap::new();
        distance_to_start_map.insert(start, 0);

        while let Some(next) = locations_to_try.pop_front() {
            let distance = distance_to_start_map[next];
            for neighbour in &neighbours[next] {
                if distance_to_start_map.get(neighbour) == None {
                    distance_to_start_map.insert(neighbour, distance + 1);
                    locations_to_try.push_back(neighbour);
                }
            }
        }

        for node in non_trivial_nodes {
            if node != start {
                if let Some(distance) = distance_to_start_map.get(node) {
                    distances.insert((*start, *node), *distance);
                }
            }
        }
    }

    // The algorithm assumes that the Volcano is connected.
    assert_eq!(
        distances.len(),
        non_trivial_nodes.len() * (non_trivial_nodes.len() - 1)
    );

    distances
}

pub struct Volcano {
    pub flows: RefCell<HashMap<Valve, i32>>,
    pub distances: HashMap<(Valve, Valve), i32>,
}

impl Volcano {
    /// Builds the network, keeping only non-trivial nodes, and including distances.
    pub fn build(input: impl Iterator<Item = String>) -> Result<Volcano, Box<dyn Error>> {
        let (flows, neighbours) = parse_input(input)?;

        let mut non_trivial_flows = HashMap::new();
        for (valve, flow) in flows.iter() {
            if *flow != 0 {
                non_trivial_flows.insert(*valve, *flow);
            }
        }
        non_trivial_flows.insert("AA".into(), 0);
        let non_trivial_nodes = HashSet::from_iter(non_trivial_flows.keys().map(|x| *x));
        let distances = build_distances(&non_trivial_nodes, &neighbours);

        Ok(Volcano {
            flows: RefCell::new(non_trivial_flows),
            distances,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_volcano_line() {
        let line = "Valve EE has flow rate=3; tunnels lead to valves FF, DD";
        let result = parse_line(line).unwrap();
        assert_eq!(result, ("EE".into(), vec!["FF".into(), "DD".into()], 3))
    }

    #[test]
    fn it_parses_volcano_line_with_one_valve() {
        let line = "Valve HH has flow rate=22; tunnel leads to valve GG";
        let result = parse_line(line).unwrap();
        assert_eq!(result, ("HH".into(), vec!["GG".into()], 22))
    }

    #[test]
    fn it_parses_a_volcano() {
        let lines = [
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ]
        .map(String::from)
        .into_iter();
        let (flows, neighbours) = parse_input(lines).unwrap();
        assert_eq!(
            flows,
            HashMap::from([
                ("AA".into(), 0),
                ("DD".into(), 20),
                ("CC".into(), 2),
                ("GG".into(), 0),
                ("JJ".into(), 21),
                ("II".into(), 0),
                ("FF".into(), 0),
                ("EE".into(), 3),
                ("HH".into(), 22),
                ("BB".into(), 13),
            ])
        );
        assert_eq!(
            neighbours[&"AA".into()],
            vec!["DD".into(), "II".into(), "BB".into()]
        );
    }

    #[test]
    fn it_builds_distances() {
        let lines = [
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ]
        .map(String::from)
        .into_iter();
        let (_, neighbours) = parse_input(lines).unwrap();
        let non_trivial_nodes = HashSet::from([
            "AA".into(),
            "BB".into(),
            "CC".into(),
            "DD".into(),
            "EE".into(),
            "HH".into(),
            "JJ".into(),
        ]);
        let distances = build_distances(&non_trivial_nodes, &neighbours);
        let expected = HashMap::from([
            (("AA".into(), "BB".into()), 1),
            (("AA".into(), "CC".into()), 2),
            (("AA".into(), "DD".into()), 1),
            (("AA".into(), "EE".into()), 2),
            (("AA".into(), "HH".into()), 5),
            (("AA".into(), "JJ".into()), 2),
            (("BB".into(), "AA".into()), 1),
            (("BB".into(), "CC".into()), 1),
            (("BB".into(), "DD".into()), 2),
            (("BB".into(), "EE".into()), 3),
            (("BB".into(), "HH".into()), 6),
            (("BB".into(), "JJ".into()), 3),
            (("CC".into(), "AA".into()), 2),
            (("CC".into(), "BB".into()), 1),
            (("CC".into(), "DD".into()), 1),
            (("CC".into(), "EE".into()), 2),
            (("CC".into(), "HH".into()), 5),
            (("CC".into(), "JJ".into()), 4),
            (("DD".into(), "AA".into()), 1),
            (("DD".into(), "BB".into()), 2),
            (("DD".into(), "CC".into()), 1),
            (("DD".into(), "EE".into()), 1),
            (("DD".into(), "HH".into()), 4),
            (("DD".into(), "JJ".into()), 3),
            (("EE".into(), "AA".into()), 2),
            (("EE".into(), "BB".into()), 3),
            (("EE".into(), "CC".into()), 2),
            (("EE".into(), "DD".into()), 1),
            (("EE".into(), "HH".into()), 3),
            (("EE".into(), "JJ".into()), 4),
            (("HH".into(), "AA".into()), 5),
            (("HH".into(), "BB".into()), 6),
            (("HH".into(), "CC".into()), 5),
            (("HH".into(), "DD".into()), 4),
            (("HH".into(), "EE".into()), 3),
            (("HH".into(), "JJ".into()), 7),
            (("JJ".into(), "AA".into()), 2),
            (("JJ".into(), "BB".into()), 3),
            (("JJ".into(), "CC".into()), 4),
            (("JJ".into(), "DD".into()), 3),
            (("JJ".into(), "EE".into()), 4),
            (("JJ".into(), "HH".into()), 7),
        ]);
        assert_eq!(distances, expected);
    }

    #[test]
    fn it_builds_non_trivial_volcano_flows() {
        let lines = [
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ]
        .map(String::from)
        .into_iter();
        let volcano = Volcano::build(lines).unwrap();
        let flows = volcano.flows.borrow().clone();
        assert_eq!(
            flows,
            HashMap::from([
                ("AA".into(), 0),
                ("DD".into(), 20),
                ("CC".into(), 2),
                ("JJ".into(), 21),
                ("EE".into(), 3),
                ("HH".into(), 22),
                ("BB".into(), 13),
            ])
        );
    }
}

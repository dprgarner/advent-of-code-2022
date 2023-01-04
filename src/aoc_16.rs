use std::{
    collections::HashMap,
    error::Error,
    io::{self, Write},
    ops::Div,
};

use crate::aoc_16::volcano::Volcano;
use itertools::Itertools;

mod volcano;

use self::volcano::Valve;

impl Volcano {
    fn solve(&self, start: Valve, time_left: i32) -> i32 {
        // println!(
        //     "Solving with start {} and remaining time {}",
        //     start, time_left
        // );
        // dbg!(&self.flows);
        let mut destinations = Vec::new();

        for (valve, flow) in self.flows.borrow().iter() {
            if start != *valve {
                let distance = self.distances[&(start, *valve)];
                if distance < time_left - 1 {
                    let cost = distance + 1;
                    let gain = (time_left - cost) * flow;
                    if gain > 0 {
                        destinations.push((*valve, cost, gain));
                    }
                }
            }
        }

        let mut max_flow = 0;
        for (valve, cost, gain) in destinations {
            let flow_value;
            {
                flow_value = self
                    .flows
                    .borrow_mut()
                    .remove(&valve)
                    .expect("Valve should be in hash map");
            }
            let potential_gain = gain + self.solve(valve, time_left - cost);
            max_flow = max_flow.max(potential_gain);
            self.flows.borrow_mut().insert(valve, flow_value);
        }

        max_flow
    }

    fn solve_with_elephant(&self, start: Valve, time_left: i32) -> i32 {
        // Splits the volcano flows into two. With 16 nodes, there are 65,536
        // partitions, which takes about 30 seconds when built with the release
        // profile. Good enough. ¯\_(ツ)_/¯
        let original_flows = self.flows.borrow().clone();

        let mut best_score = 0;
        let mut iterations = 0;
        let total_iterations = original_flows.keys().powerset().collect_vec().len();
        let step = total_iterations.div(1000);

        for elephant_flows_valves in original_flows.keys().powerset() {
            let mut elephant_flow = HashMap::new();
            for valve in elephant_flows_valves.iter() {
                elephant_flow.insert(
                    **valve,
                    self.flows
                        .borrow_mut()
                        .remove(valve)
                        .expect("Valve should be in hash map"),
                );
            }
            let my_best_score = self.solve(start, time_left);
            {
                *self.flows.borrow_mut() = elephant_flow;
            }
            let elephant_best_score = self.solve(start, time_left);
            best_score = best_score.max(my_best_score + elephant_best_score);
            *self.flows.borrow_mut() = original_flows.clone();
            iterations += 1;
            if iterations % step == 0 {
                print!(".");
                io::stdout().flush().expect("Flush should succeed");
            }
        }

        best_score
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let volcano = Volcano::build(input)?;
    let soln = volcano.solve("AA".into(), 30);
    Ok(soln)
}

pub fn solve_b(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let volcano = Volcano::build(input)?;
    let soln = volcano.solve_with_elephant("AA".into(), 26);
    Ok(soln)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_a() {
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
        let result = solve_a(lines).unwrap();
        assert_eq!(result, 1651);
    }
}

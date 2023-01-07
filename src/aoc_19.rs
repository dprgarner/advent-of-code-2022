use std::{collections::HashMap, error::Error};

use itertools::Itertools;
use regex::Regex;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(PartialEq, Eq, Debug)]
struct Blueprint {
    robot_costs: HashMap<Material, HashMap<Material, i32>>,
}

impl Blueprint {
    fn parse_cost(cost_str: &str) -> Result<HashMap<Material, i32>, Box<dyn Error>> {
        let cost_regex = Regex::new("(?P<number>\\d+) (?P<material>ore|clay|obsidian)")?;

        let mut cost = HashMap::new();
        for caps in cost_regex.captures_iter(cost_str) {
            let material = match caps.name("material").and_then(|x| Some(x.as_str())) {
                Some("ore") => Some(Material::Ore),
                Some("clay") => Some(Material::Clay),
                Some("obsidian") => Some(Material::Obsidian),
                _ => None,
            }
            .ok_or("Could not parse material")?;
            let number: i32 = caps
                .name("number")
                .ok_or("Could not parse number")?
                .as_str()
                .parse()?;
            cost.insert(material, number);
        }

        Ok(cost)
    }

    fn parse(line: String) -> Result<Blueprint, Box<dyn Error>> {
        let (ore_str, clay_str, obsidian_str, geode_str) = line
            .split(". ")
            .collect_tuple()
            .ok_or("Invalid number of robots")?;
        Ok(Blueprint {
            robot_costs: HashMap::from([
                (Material::Ore, Self::parse_cost(ore_str)?),
                (Material::Clay, Self::parse_cost(clay_str)?),
                (Material::Obsidian, Self::parse_cost(obsidian_str)?),
                (Material::Geode, Self::parse_cost(geode_str)?),
            ]),
        })
    }
}

#[derive(Clone)]
struct Factory<'a> {
    turn: i32,
    blueprint: &'a Blueprint,
    robots: HashMap<Material, i32>,
    resources: HashMap<Material, i32>,
}

impl Factory<'_> {
    fn new<'a>(blueprint: &'a Blueprint) -> Factory<'a> {
        Factory {
            turn: 0,
            blueprint,
            robots: HashMap::from([
                (Material::Ore, 1),
                (Material::Clay, 0),
                (Material::Obsidian, 0),
                (Material::Geode, 0),
            ]),
            resources: HashMap::from([
                (Material::Ore, 0),
                (Material::Clay, 0),
                (Material::Obsidian, 0),
                (Material::Geode, 0),
            ]),
        }
    }

    fn can_build_robot(&self, robot_material: &Material) -> bool {
        for (cost_material, cost_amount) in &self.blueprint.robot_costs[&robot_material] {
            if &self.resources[cost_material] < cost_amount {
                println!("Lack: {:?}", cost_material);
                return false;
            }
        }
        true
    }

    fn queue(&mut self, robots: impl Iterator<Item = Material>, turns: usize) {
        let mut robots = robots.collect_vec();

        for turn in 1..(turns + 1) {
            let mut is_building_robot = false;

            println!("Turn {turn}");
            if let Some(robot_material) = robots.first() {
                if self.can_build_robot(robot_material) {
                    is_building_robot = true;
                    println!("Building {:?} robot", robot_material);
                    for (cost_material, cost_amount) in &self.blueprint.robot_costs[&robot_material]
                    {
                        *self.resources.get_mut(cost_material).unwrap() -= cost_amount;
                    }
                }
            }
            println!("Resources:");
            for (robot_material, robot_count) in &self.robots {
                *self.resources.get_mut(robot_material).unwrap() += robot_count;
                println!(
                    "  {:?}: {}",
                    robot_material,
                    self.resources.get_mut(robot_material).unwrap()
                );
            }
            if is_building_robot {
                *self.robots.get_mut(&robots.remove(0)).unwrap() += 1;
            }
            println!("Robots:");
            for (robot_material, robot_count) in &self.robots {
                println!("  {:?}: {}", robot_material, robot_count);
            }
            println!("---");
            println!("\n");
        }
    }
}

pub fn solve_a(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let blueprints = input
        .map(Blueprint::parse)
        .collect::<Result<Vec<Blueprint>, _>>()?;

    let mut factories = blueprints.iter().map(Factory::new).collect_vec();
    factories[0].queue(
        vec![
            Material::Clay,
            Material::Clay,
            Material::Clay,
            Material::Obsidian,
            Material::Clay,
            Material::Obsidian,
            Material::Geode,
            Material::Geode,
        ]
        .into_iter(),
        24,
    );

    println!(
        "Final geodes: {}\n\n***\n",
        factories[0].resources[&Material::Geode]
    );

    Ok(1)
    // todo!("Solution for part a not yet implemented");
}

#[allow(unused_variables)]
pub fn solve_b(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    todo!("Solution for part b not yet implemented");
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_blueprint() {
        let line = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        let blueprint = Blueprint::parse(String::from(line)).unwrap();
        assert_eq!(
            blueprint,
            Blueprint {
                robot_costs: HashMap::from([
                    (Material::Ore, HashMap::from([(Material::Ore, 4)])),
                    (Material::Clay, HashMap::from([(Material::Ore, 2)])),
                    (
                        Material::Obsidian,
                        HashMap::from([(Material::Ore, 3), (Material::Clay, 14)])
                    ),
                    (
                        Material::Geode,
                        HashMap::from([(Material::Ore, 2), (Material::Obsidian, 7)])
                    ),
                ])
            }
        );
    }

    #[test]
    #[ignore]
    fn it_runs_a() {
        let input = ["aaaaa", "bbbbb"].map(String::from).into_iter();
        let result = solve_a(input).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    #[ignore]
    fn it_runs_b() {
        let input = ["aaaaa", "bbbbb"].map(String::from).into_iter();
        let result = solve_b(input).unwrap();
        assert_eq!(result, 2);
    }
}

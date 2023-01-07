use itertools::Itertools;
use regex::Regex;
use std::error::Error;

use super::material::{Material, MaterialMap};

#[derive(PartialEq, Eq, Debug)]
pub struct Blueprint {
    pub robot_costs: MaterialMap<MaterialMap<i32>>,
}

impl Blueprint {
    fn parse_cost(cost_str: &str) -> Result<MaterialMap<i32>, Box<dyn Error>> {
        let cost_regex = Regex::new("(?P<number>\\d+) (?P<material>ore|clay|obsidian)")?;

        let mut cost = MaterialMap::default();
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
            cost[&material] = number;
        }

        Ok(cost)
    }

    pub fn parse(line: String) -> Result<Blueprint, Box<dyn Error>> {
        let (ore_str, clay_str, obsidian_str, geode_str) = line
            .split(". ")
            .collect_tuple()
            .ok_or("Invalid number of robots")?;
        Ok(Blueprint {
            robot_costs: MaterialMap {
                ore: Self::parse_cost(ore_str)?,
                clay: Self::parse_cost(clay_str)?,
                obsidian: Self::parse_cost(obsidian_str)?,
                geode: Self::parse_cost(geode_str)?,
            },
        })
    }
}

use std::fmt::Display;

use super::blueprint::Blueprint;
use super::material::{Material, MaterialMap};

#[derive(Clone)]
pub struct FactoryState<'a> {
    pub turn: usize,
    max_turns: &'a usize,
    blueprint: &'a Blueprint,
    robots: MaterialMap<i32>,
    resources: MaterialMap<i32>,
}

impl Display for FactoryState<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str_ = String::new();
        str_.push_str(&format!("\nEnd of Turn {}\n", self.turn));
        str_.push_str(&format!("Resources:\n"));

        for (material, count) in &self.resources {
            str_.push_str(&format!("  {:?}: {}\n", material, count));
        }
        str_.push_str(&format!("Robots:\n"));
        for (robot_material, robot_count) in &self.robots {
            str_.push_str(&format!("  {:?}: {}\n", robot_material, robot_count));
        }

        write!(f, "{}", &str_)
    }
}

impl FactoryState<'_> {
    pub fn new<'a>(blueprint: &'a Blueprint, max_turns: &'a usize) -> FactoryState<'a> {
        FactoryState {
            max_turns,
            blueprint,
            turn: 0,
            robots: MaterialMap {
                ore: 1,
                ..Default::default()
            },
            resources: MaterialMap {
                ..Default::default()
            },
        }
    }

    /// Attempts to build this robot.
    ///
    /// Returns a factory state with the new robot if it's possible to build it
    /// within the max number of turns, otherwise returns None.
    pub fn build_next_robot<'a>(&self, robot: &Material) -> Option<Self> {
        let mut required_turns: usize = 0;
        for (cost_material, cost_amount) in &self.blueprint.robot_costs[robot] {
            if cost_amount == &0 {
                continue;
            }
            let required_turns_for_resource: usize;
            let missing_resources = cost_amount - &self.resources[&cost_material];
            let production_rate = self.robots[&cost_material];
            if production_rate == 0 {
                // There will never be enough resources to build this robot
                return None;
            }
            if missing_resources <= 0 {
                required_turns_for_resource = 0;
            } else {
                required_turns_for_resource = (missing_resources / production_rate
                    + (if missing_resources % production_rate != 0 {
                        1
                    } else {
                        0
                    })) as usize;
            }
            required_turns = required_turns.max(required_turns_for_resource);
        }
        required_turns += 1; // One more turn for the building time

        if &(self.turn + required_turns) >= self.max_turns {
            return None;
        }

        let mut new_factory = self.clone();
        for (robot_material, robot_count) in &new_factory.robots {
            let material_cost = new_factory.blueprint.robot_costs[robot][&robot_material];

            new_factory.resources[&robot_material] +=
                robot_count * required_turns as i32 - material_cost;
        }
        new_factory.turn += required_turns;
        new_factory.robots[robot] += 1;
        Some(new_factory)
    }

    pub fn score(&self) -> i32 {
        let remaining_turns = *self.max_turns as i32 - self.turn as i32;
        self.resources[&Material::Geode] + self.robots[&Material::Geode] * remaining_turns as i32
    }
}

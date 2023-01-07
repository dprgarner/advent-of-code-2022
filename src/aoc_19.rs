use std::error::Error;

mod blueprint;
mod factory_state;
mod material;

use blueprint::Blueprint;
use factory_state::FactoryState;
use material::Material;

/// Essentially brute force. Tries each material in turn as the type of next
/// robot to build, and stops if the sequence takes longer than the max turns to
/// build.
fn recursively_get_best_score(factory_states: &mut Vec<FactoryState>) -> i32 {
    let l = factory_states.len();
    let mut best_so_far = factory_states[l - 1].score();

    for material in Material::each() {
        if let Some(next_state) = factory_states[l - 1].build_next_robot(&material) {
            factory_states.push(next_state);
            best_so_far = best_so_far.max(recursively_get_best_score(factory_states));
            factory_states.pop();
        }
    }

    best_so_far
}

// Takes about 10s in the production build.
pub fn solve_a(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let blueprints = input
        .map(Blueprint::parse)
        .collect::<Result<Vec<Blueprint>, _>>()?;

    let max_turns = 24;
    let mut quality_levels = Vec::new();

    for (idx, blueprint) in blueprints.iter().enumerate() {
        let mut factory_states = Vec::from([FactoryState::new(blueprint, &max_turns)]);
        let best_score = recursively_get_best_score(&mut factory_states);
        println!("Best score for {}: {}", idx + 1, best_score);
        quality_levels.push(best_score * (idx as i32 + 1));
    }

    Ok(quality_levels.into_iter().sum())
}

// This is a scrappy brute-force solution. It took nearly an hour to run on the
// large set, and it didn't return an answer for the small set at all. (The
// small set took 10 minutes when `max_turns = 27`.)
pub fn solve_b(input: impl Iterator<Item = String>) -> Result<i32, Box<dyn Error>> {
    let max_turns = 30;
    let mut best_scores_product = 1;
    for (idx, blueprint) in input.map(Blueprint::parse).take(3).enumerate() {
        let blueprint = blueprint?;
        let mut factory_states = Vec::from([FactoryState::new(&blueprint, &max_turns)]);
        let best_score = recursively_get_best_score(&mut factory_states);
        println!("Best score for {}: {}", idx + 1, best_score);
        best_scores_product *= best_score;
    }
    Ok(best_scores_product)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aoc_19::material::{Material, MaterialMap};

    static BLUEPRINT_1 :&str  = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
    static BLUEPRINT_2 :&str  = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    fn check_soln(
        blueprint: &Blueprint,
        max_turns: &usize,
        materials: impl Iterator<Item = Material>,
    ) -> i32 {
        let mut factory_states = Vec::from([FactoryState::new(blueprint, max_turns)]);
        for material in materials {
            {
                if let Some(next_state) = factory_states.last().unwrap().build_next_robot(&material)
                {
                    println!("Built robot: {:?}", &material);
                    factory_states.push(next_state);
                } else {
                    println!("Warning: could not build robot");
                    break;
                }
                println!("{}", factory_states.last().unwrap());
            }
        }
        println!("Turns taken: {}", factory_states.last().unwrap().turn);
        while let Some(factory_state) = factory_states.pop() {
            if &factory_state.turn <= max_turns {
                let score = factory_state.score();
                println!("Score at turn {}: {}", max_turns, score);
                println!("---\n");
                return score;
            }
        }
        panic!("Could not find score");
    }

    #[test]
    fn it_parses_a_blueprint() {
        let blueprint = Blueprint::parse(String::from(BLUEPRINT_1)).unwrap();
        assert_eq!(
            blueprint,
            Blueprint {
                robot_costs: MaterialMap {
                    ore: MaterialMap {
                        ore: 4,
                        ..Default::default()
                    },
                    clay: MaterialMap {
                        ore: 2,
                        ..Default::default()
                    },
                    obsidian: MaterialMap {
                        ore: 3,
                        clay: 14,
                        ..Default::default()
                    },
                    geode: MaterialMap {
                        ore: 2,
                        obsidian: 7,
                        ..Default::default()
                    },
                }
            }
        );
    }

    #[test]
    fn it_iterates_to_short_soln_1() {
        let soln = check_soln(
            &Blueprint::parse(String::from(BLUEPRINT_1)).unwrap(),
            &24,
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
        );
        assert_eq!(soln, 9);
    }

    #[test]
    fn it_iterates_to_long_soln_1() {
        let soln = check_soln(
            &Blueprint::parse(String::from(BLUEPRINT_1)).unwrap(),
            &32,
            vec![
                Material::Ore, // I could build Clay earlier, but it's better to build this.
                Material::Clay,
                Material::Clay,
                Material::Clay,
                Material::Clay,
                Material::Clay,
                Material::Clay,
                Material::Clay, // I could build Obs earlier, but it's better to build this.
                Material::Obsidian,
                Material::Obsidian,
                Material::Obsidian,
                Material::Obsidian,
                Material::Geode,
                Material::Obsidian,
                Material::Geode,
                Material::Geode,
                Material::Geode,
                Material::Geode,
                Material::Geode,
                Material::Geode,
                Material::Geode,
                Material::Geode,
            ]
            .into_iter(),
        );
        assert_eq!(soln, 56);
    }

    #[test]
    fn it_iterates_to_short_soln_2() {
        let soln = check_soln(
            &Blueprint::parse(String::from(BLUEPRINT_2)).unwrap(),
            &24,
            vec![
                Material::Ore,
                Material::Ore,
                Material::Clay,
                Material::Clay,
                Material::Clay,
                Material::Clay,
                Material::Clay,
                Material::Obsidian,
                Material::Obsidian,
                Material::Obsidian,
                Material::Obsidian,
                Material::Obsidian,
                Material::Geode,
                Material::Obsidian,
                Material::Geode,
                Material::Geode,
            ]
            .into_iter(),
        );
        assert_eq!(soln, 12);
    }

    #[test]
    fn it_iterates_to_long_soln_2() {
        // Found by trial-and-error.
        let soln = check_soln(
            &Blueprint::parse(String::from(BLUEPRINT_2)).unwrap(),
            &32,
            vec![
                // This soln has the property that, on each turn, the best robot possible is built.
                // Nothing built on the first two turns
                Material::Ore,
                // Nothing built on this turn. After this, a robot is built on every other turn.
                Material::Ore,
                Material::Clay,
                Material::Clay,
                Material::Clay,
                Material::Clay,
                Material::Clay,
                Material::Obsidian,
                Material::Clay, // Redundant
                Material::Obsidian,
                Material::Obsidian,
                Material::Obsidian,
                Material::Clay, // Redundant
                Material::Obsidian,
                Material::Geode,
                Material::Obsidian,
                Material::Geode,
                Material::Obsidian,
                Material::Geode,
                Material::Obsidian,
                Material::Geode,
                Material::Obsidian,
                Material::Geode,
                Material::Geode,
                Material::Geode,
                Material::Obsidian, // Redundant
                Material::Geode,
                Material::Geode,
                Material::Geode, // Redundant
            ]
            .into_iter(),
        );
        assert_eq!(soln, 62);
    }
}

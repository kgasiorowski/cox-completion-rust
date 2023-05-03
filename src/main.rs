mod consts;

use std::collections::{HashMap, HashSet};
use std::time::Instant;
use rand::prelude::*;
use rand::seq::SliceRandom;

fn main() {
    let unique_weights: HashMap<&str, i8> = [
        ("Arcane Prayer Scroll", 20),
        ("Dexterous Prayer Scroll", 20),
        ("Twisted Buckler", 4),
        ("Dragon Hunter Crossbow", 4),
        ("Dinh's Bulwark", 3),
        ("Ancestral Hat", 3),
        ("Ancestral Robe Top", 3),
        ("Ancestral Robe Bottom", 3),
        ("Dragon Claws", 3),
        ("Elder Maul", 2),
        ("Kodai Insignia", 2),
        ("Twisted Bow", 2)
    ]
    .iter()
    .cloned()
    .collect();

    let mut unique_table = Vec::new();

    for (unique, quantity) in &unique_weights {
        for _ in 0..*quantity {
            unique_table.push(unique);
        }
    }
    // Our weighted unique table is generated now

    let mut completion_kcs: Vec<u64> = Vec::new();
    let mut num_tbows_by_kc_completion: u32 = 0;
    let mut num_uniques_for_completion: u8 = consts::NUM_UNIQUES;
    if consts::PET {
        num_uniques_for_completion += 1;
    }

    let start = Instant::now();
    for i in 0..consts::NUM_ACCTS {
        if (i % 10000) == 0 {
            println!("{i}")
        }
        let mut obtained_drops: HashSet<String> = HashSet::new();
        let mut obtained_tbow: bool = false;
        let mut kc_counter: u64 = 0;
        while obtained_drops.len() != num_uniques_for_completion.into() {
            kc_counter += 1;
            if thread_rng().gen::<f32>() < (consts::NUM_POINTS as f32 / 8676.0) / 100.0 {
                let drop: String = unique_table.choose(&mut thread_rng())
                    .unwrap()
                    .to_string();

                if drop == "Twisted Bow" && kc_counter < consts::TBOW_TESTING_KC.into() {
                    obtained_tbow = true;
                }

                obtained_drops.insert(drop);
                if consts::PET && thread_rng().gen::<f32>() < (1.0/53.0) {
                    obtained_drops.insert(String::from("Olmlet"));
                }
            }
        }
        if obtained_tbow {
            num_tbows_by_kc_completion += 1;
        }
        completion_kcs.push(kc_counter)
    }
    let duration = start.elapsed();
    println!("Simulation duration: {duration:?}");
    let avg_kc_for_completion: f32 = (completion_kcs.iter().sum::<u64>() as f32) / consts::NUM_ACCTS as f32;
    let percent_got_tbow_by_testing_kc: f32 = num_tbows_by_kc_completion as f32 / consts::NUM_ACCTS as f32 * 100.0;
    println!("Average completion kc: {avg_kc_for_completion}");
    println!("Percent of accounts {percent_got_tbow_by_testing_kc}% ({num_tbows_by_kc_completion}/{})", consts::NUM_ACCTS);
}

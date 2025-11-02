use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::stratagems::{Stratagem, StratagemType};

pub fn generate_loadout(stratagems: &[Stratagem]) -> Vec<Stratagem> {
    let mut rng = thread_rng();

    for _ in 0..1000 {
        let candidate: Vec<Stratatgem> = stratagems.choose_multiple(&mut rng, 4).cloned().collect();

        if is_valid_loadout(&candidate) {
            return candidate;
        }
    }

    panic!("Failed to generate a valid loadout after many attempts");
}

fn is_valid_loadout(loadout: &[Stratagem]) -> bool {
    let mut backpack_count = 0;
    let mut support_count = 0;
    let mut backpack_allows_support = false;

    for strat in loadout {
        match strat.s_type {
            StratagemType::Backpack => {
                backpack_count += 1;
                if strat.allows_support {
                    backpack_allows_support = true;
                }
            }
            StratagemType::Support => {
                support_count += 1;
            }
            _ => {}
        }
    }

    if backpack_count > 1 {
        return false;
    }
    if support_count > 1 {
        return false;
    }
    for strat in loadout {
        if strat.s_type == StratagemType::Support && !strat.allows_support {
            return backpack_count == 0 && support_count == 1;
        }
    }

    if backpack_allows_support && support_count <= 1 {
        return true;
    }
    true
}

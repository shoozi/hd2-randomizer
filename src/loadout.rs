use rand::seq::SliceRandom;
use rand::rng;

use crate::stratagems::{Stratagem, StratagemType};

pub fn generate_loadout(stratagems: &[Stratagem]) -> Vec<&Stratagem> {
    let mut rng = rng();

    // Simple sampling by shuffling indices to avoid needing extra traits.
    for _ in 0..1000 {
        if stratagems.len() < 4 {
            panic!("Not enough stratagems to generate a loadout");
        }

        let mut pool: Vec<&Stratagem> = stratagems.iter().collect();
        pool.shuffle(&mut rng);
        let candidate: Vec<&Stratagem> = pool.iter().cloned().take(4).collect();

        if is_valid_loadout(&candidate) {
            return candidate;
        }
    }

    panic!("Failed to generate a valid loadout after many attempts");
}

fn is_valid_loadout(loadout: &[&Stratagem]) -> bool {
    let mut backpack_count = 0;
    let mut support_count = 0;
    let mut backpack_allows_support = false;

    for strat in loadout {
        match strat.s_type {
            StratagemType::Backpack => {
                backpack_count += 1;
                if strat.allows_support_with_backpack {
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
        if strat.s_type == StratagemType::Support && !strat.allows_support_with_backpack {
            return backpack_count == 0 && support_count == 1;
        }
    }

    if backpack_allows_support && support_count <= 1 {
        return true;
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::stratagems;

    #[test]
    fn generate_returns_four_and_valid() {
        let pool = stratagems::get_all_stratagems();
        let loadout = generate_loadout(&pool);
        assert_eq!(loadout.len(), 4);
        assert!(is_valid_loadout(&loadout));
    }
}

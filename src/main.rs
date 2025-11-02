mod loadout;
mod stratagems;

fn main() {
    let pool = stratagems::get_all_stratagems();
    let loadout = loadout::generate_loadout(&pool);

    println!("Generated Loadout:");
    for strat in loadout {
        println!("- {}", strat.name);
    }
}

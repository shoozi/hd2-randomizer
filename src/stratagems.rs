#[derive(PartialEq, Debug)]
pub enum StratagemType {
    Support,
    Backpack,
    Orbital,
    Eagle,
    Utility,
    Vehicle,
}

pub struct Stratagem {
    pub name: &'static str,
    pub s_type: StratagemType,
    pub allows_support_with_backpack: bool,
}

pub fn get_all_stratagems() -> Vec<Stratagem> {
    vec![
        // Patriotic Administration Center
        Stratagem {
            name: "MG-43 Machine Gun",
            s_type: StratagemType::Support,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "APW-1 Anti-Material Rifle",
            s_type: StratagemType::Support,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "M-105 Stalwart",
            s_type: StratagemType::Support,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "EAT-17 Experimental Anti-Tank",
            s_type: StratagemType::Support,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "GR-8 Recoilless Rifle",
            s_type: StratagemType::Support,
            allows_support_with_backpack: false,
        },
        Stratagem {
            name: "FLAM-40 Flamethrower",
            s_type: StratagemType::Support,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "AC-88 Autocannon",
            s_type: StratagemType::Support,
            allows_support_with_backpack: false,
        },
        Stratagem {
            name: "MG-206 Heavy Machine Gun",
            s_type: StratagemType::Support,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "RL-77 Airburst Rocket Launcher",
            s_type: StratagemType::Support,
            allows_support_with_backpack: false,
        },
        Stratagem {
            name: "MLS-4X Commando",
            s_type: StratagemType::Support,
            allows_support_with_backpack: false,
        },
        Stratagem {
            name: "RS-422 Railgun",
            s_type: StratagemType::Support,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "FAF-14 Spear",
            s_type: StratagemType::Support,
            allows_support_with_backpack: false,
        },
        Stratagem {
            name: "StA-X3 W.A.S.P. Launcher",
            s_type: StratagemType::Support,
            allows_support_with_backpack: false,
        },
        // Orbital Cannons
        Stratagem {
            name: "Orbital Gatling Barrage",
            s_type: StratagemType::Orbital,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Orbital Airburst Airstrike",
            s_type: StratagemType::Orbital,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Orbital 120mm HE Barrage",
            s_type: StratagemType::Orbital,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Orbital 380mm HE Barrage",
            s_type: StratagemType::Orbital,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Orbital Walking Barrage",
            s_type: StratagemType::Orbital,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Orbital Laser",
            s_type: StratagemType::Orbital,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Orbital Napalm Barrage",
            s_type: StratagemType::Orbital,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Orbital Railcannon Strike",
            s_type: StratagemType::Orbital,
            allows_support_with_backpack: true,
        },
        // Hangar
        Stratagem {
            name: "Eagle Strafing Run",
            s_type: StratagemType::Eagle,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Eagle Airstrike",
            s_type: StratagemType::Eagle,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Eagle Cluster Bomb",
            s_type: StratagemType::Eagle,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Eagle Napalm Strike",
            s_type: StratagemType::Eagle,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "LIFT-850 Jump Pack",
            s_type: StratagemType::Backpack,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Eagle Smoke Strike",
            s_type: StratagemType::Eagle,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Eagle 110m Rocket Pods",
            s_type: StratagemType::Eagle,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Eagle 500kg Bomb",
            s_type: StratagemType::Eagle,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "M-102 Fast Recon Vehicle",
            s_type: StratagemType::Vehicle,
            allows_support_with_backpack: true,
        },
        // Bridge
        Stratagem {
            name: "Orbital Precision Strike",
            s_type: StratagemType::Orbital,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Orbital Gas Strike",
            s_type: StratagemType::Orbital,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Orbital EMS Strike",
            s_type: StratagemType::Orbital,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "Orbital Smoke Strike",
            s_type: StratagemType::Orbital,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "E/MG-101 HMG Emplacement",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "FX-12 Shield Generator Relay",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "A/ARC-3 Tesla Tower",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "E/GL-21 Grenadier Battlement",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        // Engineering Bay
        Stratagem {
            name: "MD-6 Anti Personnel Minefield",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "B-1 Supply Pack",
            s_type: StratagemType::Backpack,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "GL-21 Grenade Launcher",
            s_type: StratagemType::Support,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "LAS-98 Laser Cannon",
            s_type: StratagemType::Support,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "MD-14 Incendiary Mines",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "AX/LAS-5 Guard Dog Rover",
            s_type: StratagemType::Backpack,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "SH-20 Ballistic Shield Backpack",
            s_type: StratagemType::Backpack,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "ARC-3 Arc Thrower",
            s_type: StratagemType::Support,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "MD-17 Anti-Tank Mines",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "LAS-99 Quasar Cannon",
            s_type: StratagemType::Support,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "SH-32 Shield Generator Pack",
            s_type: StratagemType::Backpack,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "MD-8 Gas Mines",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        // Robotics Workshop
        Stratagem {
            name: "A/MG-43 Machine Gun Sentry",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "A/G-16 Gatling Sentry",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "A/M-12 Mortar Sentry",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "AX/AR-23 Guard Dog",
            s_type: StratagemType::Backpack,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "A/AC-8 Autocannon Sentry",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "A/MLS-4X Rocket Sentry",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "A/M-23 EMS Mortar Sentry",
            s_type: StratagemType::Utility,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "EXO-45 Patriot Exosuit",
            s_type: StratagemType::Vehicle,
            allows_support_with_backpack: true,
        },
        Stratagem {
            name: "EXO-49 Emancipator Exosuit",
            s_type: StratagemType::Vehicle,
            allows_support_with_backpack: true,
        },
    ]
}

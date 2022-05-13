// BSD 3-Clause License

// Copyright (c) 2022, nxtlo
// All rights reserved.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:

// 1. Redistributions of source code must retain the above copyright notice, this
// list of conditions and the following disclaimer.

// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.

// 3. Neither the name of the copyright holder nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use crate::items::{
    Item,
    ItemRarity,
    MetaData,
    Generator,
    ItemType,
};

use rand::prelude::SliceRandom;
use rand::{random, thread_rng};
use std::fmt;

static UNKNOWN: &str = &"UNKNOWN";

/// ## Weapons have ammo, And ammo have a damage type.
/// These are the available types.
/// - [`WeaponAmmoType::Toxic`]
///     - A toxic weapon that damage enemies overtime.
/// - [`WeaponAmmoType::Radiant`]
///     - A type of weapon ammo that can heal allies.
/// - [`WeaponAmmoType::Void`]
///     - A type of weapon ammo that consumes the enemy's health
///     damaging them and debuffing for 5 seconds.
/// - [`WeaponAmmoType::Elemental`]
///     - A type of weapon that can deal elemental damage. Either fire, ice, or lightning.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum WeaponAmmoType {
    Toxic,
    Radiant,
    Void,
    Elemental,
}

/**
## Core weapon types.
### Random drop weapon names for each type.
[`WeaponType::Mace`]
- `Threaded Needle`
- `Jotunn's Vigor`
- `Hydras`

[`WeaponType::Bow`]
- `Scream`
- `Sorrowbane`
- `Death's whisper`

[`WeaponType::Rod`]
- `Underlight Angler`
- `Bancrofts`
- `Arondight`
- `Hope`

[`WeaponType::Claw`]
- `Thunderlord`
- `Thorns`
- `Divine Ruin`

[`WeaponType::Daggers`]
- `Katana`
- `Wind Deamon`
- `Serrated Edge`
- `Soul Eater`
*/
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum WeaponType {
    Mace,
    Bow,
    Rod,
    Claw,
    Daggers,
}

impl Default for WeaponType {
    // Base starter weapon.
    fn default() -> Self {
        Self::Bow
    }
}

impl MetaData for WeaponType {
    fn description(&self) -> &'static str {
        match self {
            WeaponType::Bow => {
                concat!("A Standard primary bow that all players start with. ",
                "Hits with this weapon deals bonus True damage and a chance to freeze them."
                )
            },
            WeaponType::Mace => {
                concat!("A one handed heavy weapon that requires strength. ",
                "Hits with this weapon provides a chance to burn the enemy."
                )
            },
            WeaponType::Claw => {
                concat!(
                    "An powerful lethal weapon that bleedes enemies every 3 successful hits",
                )
            }
            WeaponType::Rod => "A high velocity, Ranged, Magical weapon that can stun enemies on hits.",
            WeaponType::Daggers => "Slash through enemies quickly. Rapid kills provides a chance to toxin enemies.",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            WeaponType::Bow => "Bow",
            WeaponType::Mace => "Mace",
            WeaponType::Claw => "Claw",
            WeaponType::Rod => "Rod",
            WeaponType::Daggers => "Daggers",
        }
    }
}

impl Generator for WeaponType {
    fn generate_name(&self) -> Vec<&'static str> {
        match *self {
            WeaponType::Mace => {
                return vec!["Threaded Needle", "Jotunn'a Vigor", "Hydras"];
            }
            WeaponType::Bow => {
                return vec!["Scream", "Sorrowbane", "Death's whisper"];
            }
            WeaponType::Rod => {
                return vec!["Underlight Angler", "Bancrofts", "Arondight", "Hope"];
            }
            WeaponType::Claw => {
                return vec!["Thnuderlord", "Throns", "Divine Ruin"];
            }
            WeaponType::Daggers => {
                return vec!["Katana", "Wind Deamon", "Serrated Edge", "Soul Eater"];
            }
        }
    }

    fn auto_name(&self) -> &'static str {
        let mut rng = thread_rng();
        let name = self.generate_name();
        name.choose(&mut rng).unwrap_or(&UNKNOWN)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Weapon {
    weapon_type: WeaponType,
    rarity: ItemRarity,
    name: &'static str,
    id: u8,
}

impl Default for Weapon {
    fn default() -> Self {
        let weapon_type = WeaponType::default();
        Weapon {
            weapon_type,
            rarity: random::<ItemRarity>(),
            id: random::<u8>(),
            name: weapon_type.auto_name(),
        }
    }
}

impl fmt::Debug for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Weapon(name: {}, hash: {}, type: {}, description: {})",
            self.name,
            self.id,
            self.weapon_type.name(),
            self.weapon_type.description(),
        )
    }
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Weapon(name: {}, hash: {}, type: {}, description: {})",
            self.name,
            self.id,
            self.weapon_type.name(),
            self.weapon_type.description()
        )
    }
}

impl Item for Weapon {
    fn rarity(&self) -> ItemRarity {
        self.rarity
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn id(&self) -> u8 {
        self.id
    }

    fn item_type(&self) -> crate::items::ItemType {
        ItemType::Weapon
    }
}

impl Weapon {
    pub fn new(item_type: WeaponType) -> Weapon {
        Weapon {
            name: item_type.auto_name(),
            weapon_type: item_type,
            rarity: random::<ItemRarity>(),
            id: Weapon::default().id(),
        }
    }
}

#[test]
fn test_weapon() {
    // Logging
    use log::*;
    set_max_level(LevelFilter::Info);
    env_logger::init();

    let w = Weapon::new(WeaponType::Daggers);
    println!("{}", w)
}

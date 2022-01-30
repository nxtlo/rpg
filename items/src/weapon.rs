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

use crate::item::{ItemContainer, ItemRarity};

use rand::{prelude::SliceRandom, random};
use std::fmt;

const UNKNOWN: &&str = &"UNKNOWN";

/// ## Weapons have ammo, And ammo have a damage type.
/// These are the available types.
/// - [`WeaponAmmoType::Toxic`]
///     - A toxic weapon that damage enemies overtime.
/// - [`WeaponAmmoType::Radiant`]
///     - A type of weapon ammo that can heal allies.
/// - [`WeaponAmmoType::Void`]
///     - A type of weapon ammo that consumes the enemy's health
///     damaging them and debuffing for 5 seconds.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum WeaponAmmoType {
    Toxic,
    Radiant,
    Void,
}

/**
## Core weapon types.
### Random drop weapon names for each type.
[`WeaponType::Submachine`]
- `Threaded Needle`
- `Jotunn's Vigor`
- `Hydra`

[`WeaponType::Lethalmachine`]
- `Scream`
- `Sorrowbane`
- `Death's whisper`

[`WeaponType::Magicalmachine`]
- `Underlight Angler`
- `Bancrofts`
- `Arondight`
- `Hope`

[`WeaponType::Machinegun`]
- `Thunderlord`
- `Dagger`
- `Divine Ruin`
*/
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum WeaponType {
    Submachine,
    Machinegun,
    Lethalmachine,
    Magicalmachine,
}

impl Default for WeaponType {
    /// All weapon drops are submachine by default.
    fn default() -> Self {
        Self::Submachine
    }
}

impl WeaponType {
    /// Return a minimal description about the weapon type.
    pub fn description(&self) -> &'static str {
        match *self {
            WeaponType::Submachine => "A white ammo standard primary submachine gun.",
            WeaponType::Machinegun => {
                concat!(
                    "A special ammo type machinegun that can ",
                    "randomly roll with one of `WeaponAmmoType`."
                )
            }
            WeaponType::Lethalmachine => {
                concat!(
                    "An overpowered lethal weapon that ",
                    "can randomly roll with any two `WeaponAmmoType`."
                )
            }
            WeaponType::Magicalmachine => "A high velocity, ranged, magical weapon.",
        }
    }
}

impl ToString for WeaponType {
    /// Convert [`WeaponType`] enum name to a readble [`String`]
    fn to_string(&self) -> String {
        match &Self::into(*self) {
            WeaponType::Submachine => "Submachine Gun".to_string(),
            WeaponType::Machinegun => "Machine Gun".to_string(),
            WeaponType::Lethalmachine => "Lethal Weapon".to_string(),
            WeaponType::Magicalmachine => "Magical Machine".to_string(),
        }
    }
}

fn generate_name(w: &WeaponType) -> Vec<&'static str> {
    match w {
        &WeaponType::Submachine => {
            return vec!["Threaded Needle", "Jotunn'a Vigor", "Hydra"];
        }
        &WeaponType::Lethalmachine => {
            return vec!["Scream", "Sorrowbane", "Death's whisper"];
        }
        &WeaponType::Magicalmachine => {
            return vec!["Underlight Angler", "Bancrofts", "Arondight", "Hope"];
        }
        &WeaponType::Machinegun => {
            return vec!["Thnuderlord", "Dagger", "Divine Ruin"];
        }
    }
}

/// Generates an auto weapon name depends on the type.
fn auto_name<'a>(w: &WeaponType) -> Result<&'a str, Box<dyn std::error::Error>> {
    let pending: &'a str;
    match &w {
        WeaponType::Submachine => {
            pending = generate_name(w)
                .choose(&mut rand::thread_rng())
                .unwrap_or(UNKNOWN)
        }
        WeaponType::Machinegun => {
            pending = generate_name(w)
                .choose(&mut rand::thread_rng())
                .unwrap_or(UNKNOWN)
        }
        WeaponType::Lethalmachine => {
            pending = generate_name(w)
                .choose(&mut rand::thread_rng())
                .unwrap_or(UNKNOWN)
        }
        WeaponType::Magicalmachine => {
            pending = generate_name(w)
                .choose(&mut rand::thread_rng())
                .unwrap_or(UNKNOWN)
        }
    }
    Ok(&*pending)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Weapon {
    weapon_type: WeaponType,
    rarity: ItemRarity,
    name: &'static str,
    hash: u8,
}

/// The base trait that all type of weapon implement.

impl Default for Weapon {
    fn default() -> Self {
        let hash = random::<u8>();
        let rarity = random::<ItemRarity>();
        let weapon_type = WeaponType::default();
        let name = auto_name(&weapon_type).unwrap();
        Weapon {
            weapon_type,
            rarity,
            hash,
            name,
        }
    }
}

impl fmt::Debug for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Weapon(name={}, hash={}, type={}, description={})",
            self.name,
            self.hash,
            self.weapon_type.to_string(),
            self.weapon_type.description(),
        )
    }
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Weapon(name={}, hash={}, type={}, description={})",
            self.name,
            self.hash,
            self.weapon_type.to_string(),
            self.weapon_type.description()
        )
    }
}

impl ItemContainer<WeaponType, Weapon> for Weapon {
    fn new(item_type: WeaponType) -> Weapon {
        let name = auto_name(&item_type).unwrap();
        Weapon {
            name,
            weapon_type: item_type,
            rarity: random::<ItemRarity>(),
            hash: Weapon::default().hash(),
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn hash(&self) -> u8 {
        self.hash
    }

    fn rarity(&self) -> ItemRarity {
        self.rarity
    }
}

#[test]
fn test_weapon() {
    // Logging
    use log::*;
    set_max_level(LevelFilter::Info);
    env_logger::init();

    let sub = Weapon::new(WeaponType::Submachine);
    assert_eq!(sub.weapon_type, WeaponType::Submachine);
    info!("{}", sub);

    let lethal = Weapon::new(WeaponType::Lethalmachine);
    assert_eq!(lethal.weapon_type, WeaponType::Lethalmachine);

    let machine = Weapon::new(WeaponType::Machinegun);
    assert_eq!(machine.weapon_type, WeaponType::Machinegun);

    let magical = Weapon::new(WeaponType::Magicalmachine);
    assert_eq!(magical.weapon_type, WeaponType::Magicalmachine);

    let weapons = vec![sub, lethal, machine]
        .iter()
        .take_while(move |weapon| generate_name(&weapon.weapon_type).contains(&weapon.name))
        .map(move |weapon| weapon.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    info!("{}", weapons);
}

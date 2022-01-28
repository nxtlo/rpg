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

use crate::item::ItemRarity;

use rand::{prelude::SliceRandom, random};
use std::fmt::{Debug, Display};

const UNKNOWN: &&str = &"UNKNOWN";

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum WeaponAmmoType {
    /// A toxic weapon that damage enemies overtime.
    Toxic,
    /// A type of weapon ammo that can heal allies.
    Radiant,
    /// A type of weapon ammo that consumes the enemy's health
    /// damaging them and debuffing for 5 seconds.
    Void,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum WeaponType {
    Submachine,
    Machinegun,
    Lethalmachine,
    MagicalMachine,
}

impl Default for WeaponType {
    /// All weapons are primary type by default.
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
            WeaponType::MagicalMachine => "A high velocity, ranged, magical weapon.",
        }
    }
}

impl ToString for WeaponType {
    fn to_string(&self) -> String {
        match &Self::to_owned(&self) {
            WeaponType::Submachine => "Submachine Gun".to_string(),
            WeaponType::Machinegun => "Machine Gun".to_string(),
            WeaponType::Lethalmachine => "Lethal Weapon".to_string(),
            WeaponType::MagicalMachine => "Magical Machine".to_string(),
        }
    }
}

/// Constant submachine gun names.
fn generate_name(w: &WeaponType) -> Vec<&'static str> {
    match w {
        &WeaponType::Submachine => {
            return vec!["Threaded Needle", "Jotunn'a Vigor", "Hydra"];
        }
        &WeaponType::Lethalmachine => {
            return vec!["Scream", "Sorrowbane", "Death's whisper"];
        }
        &WeaponType::MagicalMachine => {
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
        WeaponType::MagicalMachine => {
            pending = generate_name(w)
                .choose(&mut rand::thread_rng())
                .unwrap_or(UNKNOWN)
        }
    }
    Ok(&*pending)
}

pub struct Weapon {
    pub weapon_type: WeaponType,
    rarity: ItemRarity,
    name: &'static str,
    hash: u32,
}

/// The base that all type of weapon implement.
pub trait WeaponImpl {
    /// Creates a new weapon given its type.
    /// All types should be derived from `WeaponType`
    /// If the type is None it will be random.
    fn new(weapon_type: Option<WeaponType>) -> Weapon;
    /// Returns the weapon name.
    fn name(&self) -> &'static str;
    /// Returns the hash that was generated for this weapon.
    fn hash(&self) -> u32;
    /// Return a read only of the item rarity.
    fn rarity(&self) -> ItemRarity;
}

impl Default for Weapon {
    fn default() -> Self {
        let hash = random::<u32>();
        let rarity = random::<ItemRarity>();
        let weapon_type = WeaponType::default();
        let name = auto_name(&weapon_type).unwrap();
        Weapon {
            weapon_type,
            rarity,
            hash,
            name: name,
        }
    }
}

impl WeaponImpl for Weapon {
    fn new(weapon_type: Option<WeaponType>) -> Weapon {
        let wt = weapon_type.unwrap_or_default();
        let name = auto_name(&wt).unwrap();
        Weapon {
            name,
            weapon_type: wt,
            rarity: random::<ItemRarity>(),
            hash: Weapon::default().hash(),
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn hash(&self) -> u32 {
        self.hash
    }

    fn rarity(&self) -> ItemRarity {
        self.rarity
    }
}

impl Debug for Weapon {
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

impl Display for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Weapon(name={}, hash={}, type={})",
            self.name,
            self.hash,
            self.weapon_type.description()
        )
    }
}

#[test]
fn test_weapon() {
    use log::debug;
    // Default is submchine
    let sub = Weapon::new(None);
    assert_eq!(sub.weapon_type, WeaponType::Submachine);
    debug!("{}", sub);
    let lethal = Weapon::new(Some(WeaponType::Lethalmachine));
    assert_eq!(lethal.weapon_type, WeaponType::Lethalmachine);
    let machine = Weapon::new(Some(WeaponType::Machinegun));
    assert_eq!(machine.weapon_type, WeaponType::Machinegun);
    let magical = Weapon::new(Some(WeaponType::MagicalMachine));
    assert_eq!(magical.weapon_type, WeaponType::MagicalMachine);
}

//! Builtin characters implementation.
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

use components::{Health, Inventory};

pub(super) use crate::character::{BuiltinCharacter, Char, CharacterClass, MetaData};
use crate::stats::Stats;

macro impl_builtin_character($name:ident, $builtin:expr, $class:expr) {
    /// Builtin [$name] character.
    #[derive(Clone, Debug, PartialEq)]
    pub struct $name {
        health: Health,
        inventory: Inventory,
        stats: Stats,
        class: CharacterClass,
    }

    impl ::std::fmt::Display for $name {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}(class: {}, health: {})",
                $builtin.name(),
                $class.name(),
                self.health().current(),
            )
        }
    }

    impl Into<super::character::BuiltinCharacter> for $name {
        fn into(self) -> super::character::BuiltinCharacter {
            $builtin
        }
    }

    impl super::Char for $name {
        fn new() -> Self {
            let mut inventory = Inventory::default();
            // Put default starter weapon in inventory.
            inventory
                .put_weapon(crate::components::Weapon::default())
                .unwrap();

            // TODO: Each character should have different stats.
            // i,e., Vampire should have more movement speed and Assasin should have more attack speed.
            Self::build(&&inventory, &Stats::default(), &Health::default())
        }

        fn build(inventory: &Inventory, stats: &Stats, health: &Health) -> Self {
            Self {
                class: $class,
                health: health.clone(),
                inventory: inventory.clone(),
                stats: stats.clone(),
            }
        }

        fn class(&self) -> &CharacterClass {
            &self.class
        }

        fn stats(&self) -> &Stats {
            &self.stats
        }

        fn is_builtin(&self) -> bool {
            true
        }

        fn inventory(&self) -> &Inventory {
            &self.inventory
        }

        fn health(&self) -> &Health {
            &self.health
        }
    }
}

impl_builtin_character!(Vamp, BuiltinCharacter::Vamp, CharacterClass::Vampire);
impl_builtin_character!(Kain, BuiltinCharacter::Kain, CharacterClass::Warlock);
impl_builtin_character!(Susanoo, BuiltinCharacter::Susanoo, CharacterClass::Assassin);
impl_builtin_character!(Tyr, BuiltinCharacter::Tyr, CharacterClass::Warrior);

#[cfg(test)]
mod tests {

    use super::*;

    fn print(data: &impl std::fmt::Display) {
        println!("{}", data);
    }

    #[test]
    fn test_vamp() {
        let vamp = Vamp::new();
        assert!(vamp.health().current() == 100);
        assert!(!vamp.inventory().is_full());

        for item in vamp.inventory().get_weapons() {
            print(&item);
        }

        assert_eq!(vamp.stats().movement_speed, 0);
        print(&vamp);
    }

    #[test]
    fn test_build() {
        let inventory = Inventory::new();
        let health = Health::new(Some(200));
        let stats = Stats::default();
        let kain = Kain::build(&inventory, &stats, &health);
        print(&kain);

        assert!(kain.is_builtin());

        let into_builtin: BuiltinCharacter = kain.into();
        print(&into_builtin.description());
    }
}

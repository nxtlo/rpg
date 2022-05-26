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

pub(crate) use components::{
    health::Health,
    inventory::Inventory,
    items::MetaData,
    // weapon,
};
use crate::stats::Stats;

use std::fmt::Display;

/// Core character classes.
///
/// For more information run.
///
/// ```
/// use characters::{CharacterClass, components::MetaData};
///
/// let assassin = CharacterClass::Assassin;
/// println!("{}", assassin.name());
/// ```
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CharacterClass {
    Warrior,
    Warlock,
    Vampire,
    Assassin,
}

impl Default for CharacterClass {
    fn default() -> Self {
        CharacterClass::Warrior
    }
}

// Quick way to kill repeated code.
pub(crate) macro impl_builtin_character {

    ($character:ty, $builtin:expr, $class:expr) => {

        impl ::std::fmt::Display for $character {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}(health: {})", $class.name(), self.health().current())
            }
        }

        impl $character {
            /// Return information about the builtin character.
            pub fn about(&self) -> BuiltinCharacter {
                $builtin
            }

            /// Construct an empty character.
            pub fn build(health: &Health, inventory: &Inventory, stats: &Stats) -> Self {
                Self {
                    class: $class,
                    health: health.clone(),
                    inventory: inventory.clone(),
                    stats: stats.clone(),
                }
            }
        }

        impl super::Char for $character {
            /// Create a new character.
            fn new() -> Self {
                let mut inventory = Inventory::default();
                // Put default starter weapon in inventory.
                inventory.put_weapon(crate::components::Weapon::default()).unwrap();

                // TODO: Each character should have different stats.
                // i,e., Vampire should have more movement speed and Assasin should have more attack speed.
                Self::build(&Health::default(), &inventory, &Stats::default())
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
}

impl MetaData for CharacterClass {
    /// Returns the string version of the character type name.
    ///
    /// ## Returns
    /// [`String`]
    /// The character enum field name as a String.
    fn name(&self) -> &'static str {
        match self {
            CharacterClass::Warrior => "Warrior",
            CharacterClass::Warlock => "Warlock",
            CharacterClass::Vampire => "Vampire",
            CharacterClass::Assassin => "Assassin",
        }
    }

    /// Small description/lore about the character's background
    fn description(&self) -> &'static str {
        todo!()
    }
}

/// Builtin character enum.
#[derive(PartialEq, Eq, Debug)]
pub enum BuiltinCharacter {
    Tyr,
    Yemoja,
    Vamp,
    Susanoo,
}

impl MetaData for BuiltinCharacter {
    fn name(&self) -> &'static str {
        match self {
            Self::Tyr => "Tyr",
            Self::Yemoja => "Yemoja",
            Self::Vamp => "Vamp",
            Self::Susanoo => "Susanoo",
        }
    }

    /// Small description/lore about the character's background
    fn description(&self) -> &'static str {
        match self {
            Self::Tyr => {
                "The most glittering of gods, Tyr, who,
                like the Vanir, is gifted with the gift of foresight,
                and top him off with a stylish headdress."
            }
            Self::Yemoja => {
                "Mother of origins, guardian
                of passages, generator of new life in flood waters, orgasm,
                birth waters, baptism."
            }
            Self::Vamp => {
                "Lie in wait inside the walls to hunt the strays.
                Disorient your foes senses before taking their life."
            }
            Self::Susanoo => {
                "I will create a worldly paradise in this land.
            A place of peace and prosperity.
            An ideal country for those who live in suffering."
            }
        }
    }
}

/// Core trait that any character must implement from.
pub trait Char: Send + Sync {
    fn new() -> Self
    where
        Self: Sized;
    /// Returns an immutable reference to the character's inventory.
    fn inventory(&self) -> &Inventory;
    /// Returns an immutable reference to the character's health.
    fn health(&self) -> &Health;
    /// Returns an immutable reference for this character's class.
    fn class(&self) -> &CharacterClass;
    fn stats(&self) -> &Stats;
    fn is_builtin(&self) -> bool {
        false
    }
}

/// A generic character builder that can be used to create a character.
///
/// This accepts a `CharImpl` type that must be an item that implements [`Char`].
///
/// ```, ignore
/// use characters::{Char, CharImpl, Susanoo};
/// // Construct a new susanoo character.
/// let new_susanoo = Character::<Susanoo>::new();
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Character<CharImpl: Char> {
    base: CharImpl,
}

impl<CharImpl> Character<CharImpl>
where
    CharImpl: Char,
{
    #[must_use]
    pub fn new() -> CharImpl {
        <CharImpl as Char>::new()
    }
}

impl<T: Char> Display for Character<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}(health: {})",
            self.base.class().name(),
            self.base.health().current(),
        )
    }
}

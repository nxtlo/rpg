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

use crate::stats::Stats;
pub(crate) use components::{health::Health, inventory::Inventory, items::MetaData};

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
pub(crate) enum BuiltinCharacter {
    Tyr,
    Kain,
    Vamp,
    Susanoo,
}

impl MetaData for BuiltinCharacter {
    fn name(&self) -> &'static str {
        match self {
            Self::Tyr => "Tyr",
            Self::Kain => "Kain",
            Self::Vamp => "Vamp",
            Self::Susanoo => "Susanoo",
        }
    }

    /// Lore about the character's background.
    fn description(&self) -> &'static str {
        match self {
            Self::Tyr => {
                "The most glittering of gods, Tyr, who,
                like the Vanir, is gifted with the gift of foresight,
                and top him off with a stylish headdress."
            }
            Self::Kain => {
                concat!("Mother of origins, guardian of passages, generator of new life in flood waters, orgasm, birth waters, baptism.")
            }
            Self::Vamp => {
                "Lie in wait inside the walls to hunt the strays.
                Disorient your foes senses before taking their life."
            }
            Self::Susanoo => {
                concat!(
                    "I will create a worldly paradise in this land.",
                    "A place of peace and prosperity. ",
                    "An ideal country for those who live in suffering."
                )
            }
        }
    }
}

/// Core trait that any character must implement from.
pub trait Char: Send + Sync {
    /// The standard way to create a character.
    fn new() -> Self;
    /// Build a character giving it its components.
    fn build(inventory: &Inventory, stats: &Stats, health: &Health) -> Self;
    /// Returns an immutable reference to the character's inventory.
    fn inventory(&self) -> &Inventory;
    /// Returns an immutable reference to the character's health.
    fn health(&self) -> &Health;
    /// Returns an immutable reference for this character's class.
    fn class(&self) -> &CharacterClass;
    /// Returns an immutable reference to the character's stats.
    fn stats(&self) -> &Stats;
    /// Whether this character is builtin or not. Defautls to `false`.
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

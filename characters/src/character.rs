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

use components::{
    health::Health,
    inventory::Inventory,
    // weapon,
};
use std::sync::Arc;

/// Core character classes.
///
/// For more information run.
///
/// ```
/// use characters::CharacterClass;
///
/// let assassin = CharacterClass::Assassin;
/// assassin.description();
/// ```
#[derive(PartialEq, Debug)]
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

impl CharacterClass {
    /// Returns the string version of the character type name.
    ///
    /// ## Returns
    /// [`String`]
    /// The character enum field name as a String.
    pub fn name(&self) -> String {
        match self {
            CharacterClass::Warrior => "Warrior".to_string(),
            CharacterClass::Warlock => "Warlock".to_string(),
            CharacterClass::Vampire => "Vampire".to_string(),
            CharacterClass::Assassin => "Assassin".to_string(),
        }
    }

    /// Small description/lore about the character's background
    pub fn description(&self) -> String {
        "TODO".to_string()
    }
}

/// Builtin character enum.
#[derive(PartialEq, Debug, Clone)]
enum BuiltinCharacter {
    Tyr,
    Yemoja,
    Vamp,
    Susanoo,
}

impl BuiltinCharacter {
    pub fn name(&self) -> String {
        match self {
            Self::Tyr => "Tyr".to_string(),
            Self::Yemoja => "Yemoja".to_string(),
            Self::Vamp => "Vamp".to_string(),
            Self::Susanoo => "Susanoo".to_string(),
        }
    }

    /// Small description/lore about the character's background
    pub fn description(&self) -> String {
        match self {
            Self::Tyr => r#"
                The most glittering of gods, Tyr, who,
                like the Vanir, is gifted with the gift of foresight,
                and top him off with a stylish headdress.
            "#
            .to_string(),
            Self::Yemoja => r#"
                Mother of origins, guardian
                of passages, generator of new life in flood waters, orgasm,
                birth waters, baptism.
            "#
            .to_string(),
            Self::Vamp => "Lie in wait inside the walls to hunt the strays.
            Disorient your foes senses before taking their life.
            "
            .to_string(),
            Self::Susanoo => r#"
            I will create a worldly paradise in this land.
            A place of peace and prosperity.
            An ideal country for those who live in suffering..
            "#
            .to_string(),
        }
    }
}

pub trait Character {
    fn new() -> CharacterImpl;
    fn name(&self) -> String;
    fn inventory(&self) -> &Inventory;
    fn health(&self) -> &Health;
    fn description(&self) -> String;
    fn is_builtin(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct CharacterImpl<Cc = CharacterClass> {
    class: Cc,
    health: Arc<Health>,
    inventory: Arc<Inventory>,
}

impl Default for CharacterImpl {
    fn default() -> Self {
        Self {
            class: CharacterClass::default(),
            health: Arc::new(Health::default()),
            inventory: Arc::new(Inventory::default()),
        }
    }
}

impl Character for CharacterImpl {
    fn new() -> CharacterImpl {
        CharacterImpl::default()
    }

    fn name(&self) -> String {
        self.class.name()
    }

    fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    fn health(&self) -> &Health {
        &self.health
    }

    fn description(&self) -> String {
        self.class.description()
    }

    fn is_builtin(&self) -> bool {
        false
    }
}

// Builtin characters.

#[derive(PartialEq, Debug, Clone)]
pub struct Vamp;

#[derive(PartialEq, Debug, Clone)]
pub struct Yemoja;

#[derive(PartialEq, Debug, Clone)]
pub struct Susanoo;

#[derive(PartialEq, Debug, Clone)]
pub struct Tyr;

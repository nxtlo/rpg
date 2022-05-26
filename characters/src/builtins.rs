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

use crate::{
    character::{BuiltinCharacter, Char, CharacterClass},
    components::{Health, Inventory, Weapon},
};

/// Builtin vamp character.
#[derive(Debug, Clone, PartialEq)]
pub struct Vamp {
    class: CharacterClass,
    health: Health,
    inventory: Box<Inventory>,
}

impl std::fmt::Display for Vamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vamp(health: {})", self.health.current())
    }
}

impl Vamp {
    /// Return information about the builtin vamp character.
    pub fn about(&self) -> BuiltinCharacter {
        BuiltinCharacter::Vamp
    }

    /// Construct an empty vampire character.
    pub fn build(health: Health, inventory: &Inventory) -> Self {
        Self {
            class: CharacterClass::Vampire,
            health,
            inventory: box inventory.clone(),
        }
    }
}

impl Char for Vamp {
    /// Create a new vampire character.
    fn new() -> Vamp {
        let mut inventory = Inventory::default();
        // Put default starter weapon in inventory.
        inventory.put_weapon(Weapon::default()).unwrap();
        Vamp::build(Health::default(), &inventory)
    }

    fn class(&self) -> &CharacterClass {
        &self.class
    }

    fn is_builtin(&self) -> bool {
        true
    }

    fn inventory(&self) -> Inventory {
        *self.inventory.clone()
    }

    fn health(&self) -> &Health {
        &self.health
    }
}

// TODO: impl these.

/// Builtin yemoja character.
pub struct Yemoja;

/// Builtin Susanoo character.
pub struct Susanoo;

/// Builtin Tyr character.
pub struct Tyr;

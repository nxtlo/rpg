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
    weapon::Weapon,
    // weapon,
};
use std::cell::Cell;
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
                r#"
                The most glittering of gods, Tyr, who,
                like the Vanir, is gifted with the gift of foresight,
                and top him off with a stylish headdress.
            "#
            }
            Self::Yemoja => {
                r#"
                Mother of origins, guardian
                of passages, generator of new life in flood waters, orgasm,
                birth waters, baptism.
            "#
            }
            Self::Vamp => {
                "Lie in wait inside the walls to hunt the strays.
            Disorient your foes senses before taking their life.
            "
            }
            Self::Susanoo => {
                r#"
            I will create a worldly paradise in this land.
            A place of peace and prosperity.
            An ideal country for those who live in suffering..
            "#
            }
        }
    }
}

/// Core trait that all characters must implement from.
pub trait Character: Send {
    // Is this needed to send the char to other threads?
    fn new() -> Self
    where
        Self: Sized;
    fn inventory(&mut self) -> &mut Inventory;
    fn health(&self) -> &Health;
    fn class(&self) -> &CharacterClass;
    fn is_builtin(&self) -> bool {
        false
    }
}

impl Display for Box<dyn Character> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.class().name(), self.health())
    }
}

#[derive(Debug, Clone)]
pub struct CharacterBuilder {
    class: CharacterClass,
    health: Health,
    inventory: Inventory,
}

/// A character builder object.
impl CharacterBuilder {
    pub fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            class: CharacterClass::default(),
            health: Health::default(),
            inventory: Inventory::new(),
        }
    }

    pub fn class(&mut self, class: CharacterClass) -> &mut Self {
        self.class = class;
        self
    }

    pub fn health(&mut self, health: &Health) -> &mut Self {
        self.health = *health;
        self
    }

    pub fn inventory(&mut self, inventory: &Inventory) -> &mut Self {
        self.inventory = inventory.to_owned();
        self
    }

    pub fn finish(&self) -> Box<dyn Character> {
        match self.class {
            CharacterClass::Vampire => Box::new(Vamp::build(&self.health, &self.inventory)),
            _ => todo!(),
        }
    }
}

/// Builtin vampire character.
pub struct Vamp {
    class: CharacterClass,
    health: Health,
    inventory: Cell<Inventory>,
}

impl std::fmt::Display for Vamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Vamp(description: {}, health: {})",
            self.about().description(),
            self.health()
        )
    }
}

// Base impl that all builtin characters impls.
impl Vamp {
    /// Return information about the builtin vamp character.
    pub fn about(&self) -> BuiltinCharacter {
        BuiltinCharacter::Vamp
    }

    /// Build an empty vampire character.
    pub fn build(health: &Health, inventory: &Inventory) -> Self {
        Self {
            class: CharacterClass::Vampire,
            health: health.to_owned(),
            inventory: Cell::new(inventory.to_owned()),
        }
    }
}

impl Character for Vamp {
    /// Create a new vampire character.
    fn new() -> Vamp {
        let mut inventory = Inventory::default();
        inventory.put_weapon(Weapon::default()).unwrap();
        Vamp::build(&Health::default(), &inventory)
    }

    fn class(&self) -> &CharacterClass {
        &self.class
    }

    fn is_builtin(&self) -> bool {
        true
    }

    fn inventory(&mut self) -> &mut Inventory {
        self.inventory.get_mut()
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

#[cfg(test)]
mod test {
    use components::items::Item;
    use components::items::ItemType;

    use super::*;

    #[test]
    fn test_vamp_build() {
        let mut vamp = CharacterBuilder::new()
            .class(CharacterClass::Vampire)
            .inventory(&Inventory::new())
            .health(&Health::new(Some(50)))
            .finish();
        println!("{}", vamp.inventory());
    }

    #[test]
    fn test_vamp_basic() {
        let mut vamp = Vamp::new();
        println!("{}", vamp);
        assert_eq!(vamp.class(), &CharacterClass::Vampire);
        assert_eq!(vamp.health().get_health(), &100);
        for weapon in vamp
            .inventory()
            .get_weapons()
            .iter()
            .find(|w| w.item_type() == ItemType::Weapon)
        {
            println!("{}", weapon.name());
        }
    }
}

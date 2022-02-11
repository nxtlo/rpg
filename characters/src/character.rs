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

use items::components::Health;

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
#[derive(PartialEq, Debug, Clone)]
pub enum CharacterClass {
    Warrior,
    Warlock,
    Vampire,
    Assassin,
}

impl CharacterClass {
    /// Returns the string version of the character type name.
    ///
    /// ## Returns
    /// [`String`]
    /// The character enum field name as a String.
    pub fn name(&self) -> String {
        "TODO".to_string()
    }

    /// Small description/lore about the character's background
    pub fn description(&self) -> String {
        "TODO".to_string()
    }
}

/// Builtin character enum.
#[derive(PartialEq, Debug, Clone)]
enum BuiltinChar {
    Tyr,
    Yemoja,
    Vamp,
    Susanoo,
}

impl PartialEq<&mut BuiltinChar> for BuiltinChar {
    fn ne(&self, other: &&mut BuiltinChar) -> bool {
        self == other
    }

    fn eq(&self, other: &&mut BuiltinChar) -> bool {
        self == other
    }
}

#[allow(dead_code)]
impl BuiltinChar {
    /// Returns the string version of the character type name.
    ///
    /// ## Returns
    /// [`String`]
    /// The character enum field name as a String.
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

/// Core trait that all non-builtin characters should
/// implement from.
pub trait Character<Cc = CharacterClass>: Send + 'static {
    /// Creates a new character
    ///
    /// ## Returns
    /// [`CharacterImpl`]
    /// A standard character implementation.
    #[must_use]
    fn new() -> CharacterImpl<Cc>;
    /// The character's name.
    ///
    /// ## Returns
    /// [`String`]
    /// The character name as a str.
    fn name() -> String;
    /// A minimal description about the character.
    ///
    /// ## Returns
    /// [`String`]
    /// Some description about the character.
    fn description() -> String;
}

/// Core trait that all builtin characters should implement.
trait BuiltinCharacter<Cc = CharacterClass>: Send + 'static {
    #[must_use]
    fn new() -> BuiltinCharacterImpl<Cc>;
}

/// Core character struct returned for all
/// newly created characters.
///
/// ## About Characters
/// Characters are the core objects a player can play with, They can hold
/// weapons, consumebles, items, Use and fight with those items.
///
/// ## Making your own character.
///
/// ```
/// use characters::*;
///
/// struct Demon;
///
/// impl Character for Demon {
///     fn new() -> CharacterImpl<CharacterClass> {
///         CharacterImpl{class: CharacterClass::Vampire}
///     }
///     fn name() -> String {"Alucard".to_string()}
///     fn description() -> String {"Some description".to_string()}
/// }
/// ```
///
#[derive(PartialEq, Debug, Clone)]
pub struct CharacterImpl<Cc = CharacterClass> {
    pub class: Cc,
}

// Default character impls are not builtins.
impl CharacterImpl {
    pub fn is_builtin() -> bool {
        false
    }
}

/// Core builtin characters struct.
///
/// This struct is special and only returned by builtin characters.
#[derive(PartialEq, Debug, Clone)]
struct BuiltinCharacterImpl<Cc = CharacterClass> {
    class: Cc,
    builtin: Box<BuiltinChar>,
    health: Health,
}
// Builtin characters.

#[allow(dead_code)]
impl BuiltinCharacterImpl {
    pub fn is_builtin() -> bool {
        true
    }

    pub fn class(&self) -> CharacterClass {
        self.class.clone()
    }

    pub fn name(&self) -> String {
        self.builtin.name()
    }

    pub fn description(&self) -> String {
        self.builtin.description()
    }

    pub fn repr(&self) -> String {
        format!(
            "BuiltinCharacterImpl(name: {}, description: {}, class: {})",
            &self.name(),
            &self.description(),
            &self.class.name()
        )
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Vamp;

impl BuiltinCharacter for Vamp {
    #[must_use]
    fn new() -> BuiltinCharacterImpl {
        BuiltinCharacterImpl {
            class: CharacterClass::Vampire,
            builtin: box BuiltinChar::Vamp,
            health: Health::new(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Yemoja;

impl BuiltinCharacter for Yemoja {
    #[must_use]
    fn new() -> BuiltinCharacterImpl {
        BuiltinCharacterImpl {
            class: CharacterClass::Warlock,
            builtin: box BuiltinChar::Yemoja,
            health: Health::new(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Susanoo;

impl BuiltinCharacter for Susanoo {
    #[must_use]
    fn new() -> BuiltinCharacterImpl {
        BuiltinCharacterImpl {
            class: CharacterClass::Assassin,
            builtin: box BuiltinChar::Susanoo,
            health: Health::new(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Tyr;

impl BuiltinCharacter for Tyr {
    #[must_use]
    fn new() -> BuiltinCharacterImpl {
        BuiltinCharacterImpl {
            class: CharacterClass::Warrior,
            builtin: box BuiltinChar::Tyr,
            health: Health::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn test_vamp() {
        let mut vamp = Vamp::new();
        println!("{}", vamp.repr());

        println!("{}", vamp.health.get_health());

        vamp.health.drip_random();
        println!("{}", vamp.health);

        vamp.health.incr(10);
        println!("{}", vamp.health);
        vamp.health.drip(2);
        println!("{}", vamp.health.regen());
        println!("{}", vamp.health);

        assert_eq!(vamp.class, CharacterClass::Vampire);
        assert_eq!(vamp.builtin, box BuiltinChar::Vamp);
    }

    #[test]
    fn test_yemoja() {
        let yemoja = Yemoja::new();
        println!("{}", yemoja.repr());
        assert_eq!(yemoja.class, CharacterClass::Warlock);
        assert_eq!(yemoja.name(), "Yemoja");
        assert_eq!(yemoja.builtin, box BuiltinChar::Yemoja);
    }

    #[test]
    fn test_susanoo() {
        let sus = Susanoo::new();
        println!("{}", sus.repr());
        assert_eq!(sus.name(), "Susanoo");
        assert_eq!(sus.class, CharacterClass::Assassin);
        assert_eq!(sus.builtin, box BuiltinChar::Susanoo)
    }

    #[test]
    fn test_tyr() {
        let tyr = Tyr::new();
        println!("{}", tyr.borrow().repr());
        assert_eq!(tyr.class, CharacterClass::Warrior);
        assert_eq!(tyr.builtin, box BuiltinChar::Tyr);
    }
}

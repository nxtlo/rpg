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

//! A crate includes all components a character can have. i.e., Inventory, Health, etc.

use crate::weapon::Weapon;

/// Core object inventory component.
///
/// This includes weapons items it ownns, cosmetics, etc.
#[derive(Clone, Debug)]
pub struct Inventory {
    weapons: Vec<Weapon>,
    max_size: u32,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            weapons: vec![],
            max_size: 50,
        }
    }
}

impl Inventory {
    /// Creates a new inventory object.
    pub fn new() -> Inventory {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        !self.is_full()
    }

    pub fn is_full(&self) -> bool {
        self.max_size == (self.weapons.len() + 1) as u32
    }

    pub fn get_weapons(&self) -> Vec<Weapon> {
        self.weapons.to_vec()
    }

    pub fn put_weapon(&mut self, weapon: &Weapon) -> anyhow::Result<()> {
        if self.is_full() {
            return Err(anyhow::anyhow!("Inventory is full"));
        }

        self.weapons.push(weapon.to_owned());

        Ok(())
    }
}

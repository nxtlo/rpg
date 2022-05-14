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

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ItemRarity {
    Rare,
    Legendry,
    Exotic,
}

impl Distribution<ItemRarity> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ItemRarity {
        match rng.gen_range(0..=2) {
            0 => ItemRarity::Rare,
            1 => ItemRarity::Legendry,
            _ => ItemRarity::Exotic,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ItemType {
    Weapon,
    Container,
    Consumable,
    Armor,
}

impl Generator for ItemType {
    fn generate_name(&self) -> Vec<&'static str> {
        match &self {
            ItemType::Weapon => todo!(),
            ItemType::Container => todo!(),
            ItemType::Consumable => todo!(),
            ItemType::Armor => todo!(),
        }
    }

    fn auto_name(&self) -> &'static str {
        let _pending: &'static str;
        match self {
            ItemType::Weapon => todo!(),
            ItemType::Container => todo!(),
            ItemType::Consumable => todo!(),
            ItemType::Armor => todo!(),
        }
    }
}

/// Types that have a name and a description.
pub trait MetaData {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

/// Types that are able to generate names depend on the type of the item.
pub trait Generator {
    fn generate_name(&self) -> Vec<&'static str>;
    fn auto_name(&self) -> &'static str;
}

pub trait Item {
    fn name(&self) -> &'static str;
    fn id(&self) -> u8;
    fn item_type(&self) -> ItemType;
    fn rarity(&self) -> ItemRarity;
}

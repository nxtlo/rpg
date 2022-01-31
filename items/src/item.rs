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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ItemType {
    Weapon,
    Container,
    Consumable,
    Armor,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Item {
    name: &'static str,
    item_type: ItemType,
    rarity: ItemRarity,
    hash: u8,
}

impl ItemContainer<ItemType, Item> for Item {
    fn new(item_type: ItemType) -> Item {
        todo!()
    }

    fn name(&self) -> &'static str {
        todo!()
    }

    fn hash(&self) -> u8 {
        todo!()
    }

    fn rarity(&self) -> ItemRarity {
        todo!()
    }
}

/// Core trait that all items should implement.
/// ## NOTE
/// [`Item`] is just an item. Which can be one of [`ItemType`].
///
/// ## Generic Items.
/// This trait takes two generic types `<IT, RT>`.
///
/// First is the `Item Type`, This should be an enum
/// contain all types of your item, i.e., [`crate::weapon::WeaponType`], [`ItemType`] and others.
///
/// The second is `RT` which's the return type of the item. This will always be the items itself.
///
/// i.e., [`Item`], [`crate::weapon::Weapon`]
///
/// ## Example
/// ```
/// use items::item::{ItemContainer, ItemRarity};
///
/// struct MyItem {
///     name: &'static str,
///     item_type: MyItemType,
///     rarity: ItemRarity,
///     hash: u8
/// }
///
/// enum MyItemType {
///     Freezer,
///     Solar
/// }
///
/// impl ItemContainer<MyItemType, MyItem> for MyItem {
///     fn new(item_type: MyItemType) -> MyItem {
///         let hash = rand::random::<u8>();
///         let rarity = rand::random::<ItemRarity>();
///         let name = "Some name";
///         MyItem{name, item_type, rarity, hash}
///     }
/// }
/// ```
pub trait ItemContainer<IT, RT>: Send + 'static {
    /// Creates a new item given its type.
    fn new(item_type: IT) -> RT;
    fn name(&self) -> &'static str;
    fn hash(&self) -> u8;
    fn rarity(&self) -> ItemRarity;
}

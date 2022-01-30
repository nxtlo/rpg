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

impl Distribution<ItemRarity> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ItemRarity {
        match rng.gen_range(0..=2) {
            0 => ItemRarity::Rare,
            1 => ItemRarity::Legendry,
            _ => ItemRarity::Exotic,
        }
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
pub trait ItemContainer<IT, RT> {
    /// Creates a new item given its type.
    fn new(item_type: IT) -> RT;
    fn name(&self) -> &'static str;
    fn hash(&self) -> u8;
    fn rarity(&self) -> ItemRarity;
}

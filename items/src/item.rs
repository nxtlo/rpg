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

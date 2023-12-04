use std::ops::BitAnd;

use strum::{EnumIter, IntoEnumIterator};

pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq, EnumIter, Copy, Clone)]
pub enum Allergen {
    Eggs = 0,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.bitand(2u32.pow((*allergen) as u32)) > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iter().map(|a| {
            if self.is_allergic_to(&a) {
                Some(a)
            } else {
                None
            }
        }).flatten().collect()
    }
}

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

use rand::{thread_rng, Rng};
use std::{cell, thread, time};

pub struct Inventory {
    items: Vec<super::item::Item>,
    weapons: Vec<super::weapon::Weapon>,
}

/// Health bar for characters and other objects that can live.
#[derive(PartialEq, Debug, Clone)]
pub struct Health {
    current_health: cell::Cell<u8>,
}

unsafe impl Sync for Health {}
unsafe impl Send for Health {}

impl PartialOrd<Health> for Health {
    fn lt(&self, other: &Health) -> bool {
        self.current_health.get() < other.current_health.get()
    }

    fn le(&self, other: &Health) -> bool {
        // Pattern `Some(Less | Eq)` optimizes worse than negating `None | Some(Greater)`.
        // FIXME: The root cause was fixed upstream in LLVM with:
        // https://github.com/llvm/llvm-project/commit/9bad7de9a3fb844f1ca2965f35d0c2a3d1e11775
        // Revert this workaround once support for LLVM 12 gets dropped.
        self.current_health.get() <= other.current_health.get()
    }

    fn gt(&self, other: &Health) -> bool {
        self.current_health.get() > other.current_health.get()
    }

    fn ge(&self, other: &Health) -> bool {
        self.current_health.get() >= other.current_health.get()
    }

    fn partial_cmp(&self, other: &Health) -> Option<std::cmp::Ordering> {
        self.current_health.partial_cmp(&other.current_health)
    }
}

impl std::fmt::Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Health(current_health: {})", self.current_health.get())
    }
}

impl Health {
    /// Creates a new health bar for an object.
    pub fn new() -> Health {
        // Default start health is 100
        Health {
            current_health: cell::Cell::new(100),
        }
    }

    pub fn regen(&mut self) -> u8 {
        let mut curr_health = self.current_health.get();

        loop {
            if curr_health == 100 {
                println!("Finished");
                break;
            };

            curr_health += 1;
            thread::sleep(time::Duration::from_nanos(5));
        }
        self.current_health.set(curr_health);
        curr_health
    }

    /// Drip this health.
    /// ## Returns
    /// [`u8`] The current dripped health bar.
    pub fn drip(&mut self, by: u8) -> u8 {
        let curr_health = self.current_health.get_mut();
        *curr_health -= by;
        *curr_health
    }

    /// Drip this health randomly.
    /// ## Returns
    /// [`u8`] The current dripped health bar.
    pub fn drip_random(&mut self) -> u8 {
        let curr_health = self.current_health.get_mut();

        let range: u8 = thread_rng().gen_range(0..=*curr_health / 2).into();
        *curr_health -= range;

        *curr_health
    }

    /// Increment this health bar.
    ///
    /// ## Parameters
    /// by: [`Option<u8>`]
    ///
    /// If not set to [`None`], It will increments the health by this number. otherwise randomly.
    pub fn incr(&mut self, by: u8) -> u8 {
        let curr_health = *self.current_health.get_mut();

        // Check if we're not dead nor at max health bar.
        if self._validate() {
            self.current_health.set(curr_health + by)
        }
        curr_health
    }

    /// Increment this health bar randomly.
    pub fn incr_random(&mut self) -> u8 {
        let mut curr_health = *self.current_health.get_mut();

        // Check if we're not dead nor at max health bar.
        if self._validate() {
            let range: u8 = thread_rng().gen_range(10..=curr_health * 2).into();
            curr_health += range;

            self.current_health.set(curr_health);
        }
        curr_health
    }

    /// Kill this health bar by setting it to 0 and drop its value.
    pub fn kill(&self) {
        self.current_health.set(0);
        drop(self.current_health.get());
    }

    /// Whether this health is killed or not.
    pub fn is_killed(&self) -> bool {
        self.current_health.get().eq(&0)
    }

    /// The current health for an object.
    ///
    /// ## Returns
    /// [`u8`] The current health.
    pub fn get_health(&self) -> u8 {
        return self.current_health.get();
    }

    fn _validate(&self) -> bool {
        let curr_health = self.current_health.get();
        !curr_health >= 100 || !curr_health <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health() {
        let mut health = Health::new();
        health.drip_random();
        print!("{}", health.get_health());
    }
}

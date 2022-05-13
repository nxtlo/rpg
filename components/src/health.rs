use rand::{thread_rng, Rng};
use std::{thread, time};

/// Core health component for objects that can live.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Health {
    current_health: u32,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            current_health: 100,
        }
    }
}

impl PartialOrd<Health> for Health {
    fn lt(&self, other: &Health) -> bool {
        self.current_health < other.current_health
    }

    fn le(&self, other: &Health) -> bool {
        self.current_health <= other.current_health
    }

    fn gt(&self, other: &Health) -> bool {
        self.current_health > other.current_health
    }

    fn ge(&self, other: &Health) -> bool {
        self.current_health >= other.current_health
    }

    fn partial_cmp(&self, other: &Health) -> Option<std::cmp::Ordering> {
        self.current_health.partial_cmp(&other.current_health)
    }
}

impl std::fmt::Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Health(current_health: {})", self.get_health())
    }
}

impl Health {
    /// Creates a new health bar for an object.
    pub fn new() -> Health {
        // Default start health is 100
        Health::default()
    }

    pub fn regen(&mut self) {
        let mut slf = self.clone();

        thread::spawn(move || loop {
            if slf.ok() {
                break;
            }
            thread::sleep(time::Duration::from_millis(0.450 as u64));
            slf.incr(1).unwrap();
        })
        .join()
        .unwrap()
    }

    /// Drip this health.
    /// ## Returns
    /// [`u32`] The current dripped health bar.
    pub fn drip(&mut self, by: u32) -> u32 {
        self.current_health -= by;
        self.current_health
    }

    /// Drip this health randomly.
    /// ## Returns
    /// [`u32`] The current dripped health bar.
    pub fn drip_random(&mut self) -> u32 {
        let mut curr_health = self.current_health;

        let range: u32 = thread_rng().gen_range(0..=curr_health / 2);
        curr_health -= range;

        curr_health
    }

    /// Increment this health bar.
    ///
    /// ## Parameters
    /// by: [u32]
    pub fn incr(&mut self, by: u32) -> anyhow::Result<u32, &str> {
        // Check if we're not dead nor at max health bar.
        if self.is_killed() {
            return Err("Health is dead.");
        }

        if self.validate() {
            self.current_health += by;
        }
        Ok(self.current_health)
    }

    /// Increment this health bar randomly.
    pub fn incr_random(&mut self) -> anyhow::Result<u32, &str> {
        let mut curr_health = self.current_health;

        if self.is_killed() {
            return Err("Cannot increment health when dead.");
        }

        // Check if we're not dead nor at max health bar.
        if self.validate() {
            let range: u32 = thread_rng().gen_range(1..=curr_health / 2);
            curr_health += range;

            self.current_health = curr_health;
        }
        Ok(curr_health)
    }

    /// Kill this health bar by setting it to 0 and drop its value.
    pub fn kill(&mut self) {
        self.set_health(0);
    }

    /// Check if the health bar is at >=100.
    pub fn ok(&self) -> bool {
        self.current_health >= 100
    }

    pub fn revive(&mut self) -> anyhow::Result<bool> {
        if self.is_killed() {
            self.set_health(100);
            Ok(true)
        } else {
            Err(anyhow::anyhow!("Character is already alive!"))
        }
    }

    /// Whether this health is killed or not.
    pub fn is_killed(&self) -> bool {
        self.current_health == 0
    }

    /// Returns a reference for the current health.
    ///
    /// ## Returns
    /// [`u32`] The current health.
    pub fn get_health(&self) -> &u32 {
        &self.current_health
    }

    fn set_health(&mut self, health: u32) {
        self.current_health = health;
    }

    fn validate(&self) -> bool {
        let curr_health = self.get_health();
        !curr_health >= 100 || !curr_health <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_regen() {
        let mut health = Health::new();
        assert_eq!(health.get_health(), &100);

        health.drip(10);
        assert_eq!(health.get_health(), &90);
        println!("{}", health);

        health.regen();
        let h = *health.get_health();
        assert!(h <= 100 && h >= 90);
    }

    #[test]
    fn test_basic() {
        let mut health = Health::new();
        assert_eq!(health.get_health(), &100);

        health.drip(10);
        assert_eq!(health.get_health(), &90);
        println!("{}", health);

        health.incr(10).unwrap();
        assert_eq!(health.get_health(), &100);
        println!("{}", health);

        let new_health = health.incr_random().unwrap();
        assert_eq!(health.get_health(), &new_health);
        println!("{}", health);

        health.kill();
        assert_eq!(health.get_health(), &0);
        println!("{}", health);
    }

    #[test]
    fn test_health_incr_when_dead() {
        let mut health = Health::new();
        health.kill();
        assert_eq!(health.get_health(), &0);

        health.incr(10).unwrap_or(0);
        assert_eq!(health.get_health(), &0);
    }
}

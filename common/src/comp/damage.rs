use crate::comp::Loadout;
use serde::{Deserialize, Serialize};

pub const BLOCK_EFFICIENCY: f32 = 0.9;

pub struct Damage {
    pub healthchange: f32,
    pub source: DamageSource,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageSource {
    Melee,
    Healing,
    Projectile,
    Explosion,
    Falling,
    Shockwave,
    Energy,
}

impl Damage {
    pub fn modify_damage(&mut self, block: bool, loadout: &Loadout) {
        match self.source {
            DamageSource::Melee => {
                // Critical hit
                let mut critdamage = 0.0;
                if rand::random() {
                    critdamage = self.healthchange * 0.3;
                }
                // Block
                if block {
                    self.healthchange *= 1.0 - BLOCK_EFFICIENCY
                }
                // Armor
                let damage_reduction = loadout.get_damage_reduction();
                self.healthchange *= 1.0 - damage_reduction;

                // Critical damage applies after armor for melee
                if (damage_reduction - 1.0).abs() > f32::EPSILON {
                    self.healthchange += critdamage;
                }
            },
            DamageSource::Projectile => {
                // Critical hit
                if rand::random() {
                    self.healthchange *= 1.2;
                }
                // Block
                if block {
                    self.healthchange *= 1.0 - BLOCK_EFFICIENCY
                }
                // Armor
                let damage_reduction = loadout.get_damage_reduction();
                self.healthchange *= 1.0 - damage_reduction;
            },
            DamageSource::Explosion => {
                // Block
                if block {
                    self.healthchange *= 1.0 - BLOCK_EFFICIENCY
                }
                // Armor
                let damage_reduction = loadout.get_damage_reduction();
                self.healthchange *= 1.0 - damage_reduction;
            },
            DamageSource::Shockwave => {
                // Armor
                let damage_reduction = loadout.get_damage_reduction();
                self.healthchange *= 1.0 - damage_reduction;
            },
            DamageSource::Energy => {
                // Armor
                let damage_reduction = loadout.get_damage_reduction();
                self.healthchange *= 1.0 - damage_reduction;
            },
            _ => {},
        }
    }
}

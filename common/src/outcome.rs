use crate::comp;
use comp::item::Reagent;
use serde::{Deserialize, Serialize};
use vek::*;

/// An outcome represents the final result of an instantaneous event. It implies
/// that said event has already occurred. It is not a request for that event to
/// occur, nor is it something that may be cancelled or otherwise altered. Its
/// primary purpose is to act as something for frontends (both server and
/// client) to listen to in order to receive feedback about events in the world.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Outcome {
    Explosion {
        pos: Vec3<f32>,
        power: f32,
        radius: f32,
        is_attack: bool,
        reagent: Option<Reagent>, // How can we better define this?
    },
    ProjectileShot {
        pos: Vec3<f32>,
        body: comp::Body,
        vel: Vec3<f32>,
    },
}

impl Outcome {
    pub fn get_pos(&self) -> Option<Vec3<f32>> {
        match self {
            Outcome::Explosion { pos, .. } => Some(*pos),
            Outcome::ProjectileShot { pos, .. } => Some(*pos),
        }
    }
}

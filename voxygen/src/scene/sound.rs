use crate::audio::AudioFrontend;
use client::Client;
use common::comp::{Body, CharacterState, MovementState::*, Ori, Pos};
use common::terrain::biome::BiomeKind;
use hashbrown::HashMap;
use specs::{Entity as EcsEntity, Join};
use std::time::Instant;
use vek::*;

pub struct AnimState {
    last_step_sound: Instant,
}

pub struct SoundMgr {
    current_biome: BiomeKind,
    current_music: usize,
    character_states: HashMap<EcsEntity, AnimState>,
}

impl SoundMgr {
    pub fn new() -> Self {
        Self {
            current_biome: BiomeKind::Void,
            current_music: 0,
            character_states: HashMap::new(),
        }
    }

    pub fn maintain(&mut self, audio: &mut AudioFrontend, client: &Client) {
        let ecs = client.state().ecs();
        // Get player position.
        let player_pos = ecs
            .read_storage::<Pos>()
            .get(client.entity())
            .map_or(Vec3::zero(), |pos| pos.0);

        let player_ori = ecs
            .read_storage::<Ori>()
            .get(client.entity())
            .map_or(Vec3::zero(), |pos| pos.0);

        audio.set_listener_pos(&player_pos, &player_ori);

        for (entity, pos, body, character) in (
            &ecs.entities(),
            &ecs.read_storage::<Pos>(),
            &ecs.read_storage::<Body>(),
            ecs.read_storage::<CharacterState>().maybe(),
        )
            .join()
        {
            if let (Body::Humanoid(_), Some(character)) = (body, character) {
                let state = self
                    .character_states
                    .entry(entity)
                    .or_insert_with(|| AnimState {
                        last_step_sound: Instant::now(),
                    });

                if character.movement == Run && state.last_step_sound.elapsed().as_secs_f64() > 0.25
                {
                    let rand_step = (rand::random::<usize>() % 5) + 1;
                    audio.play_sound(
                        &format!("voxygen.audio.sfx.steps.step_{}", rand_step),
                        pos.0,
                    );
                    state.last_step_sound = Instant::now();
                }
            }
        }

        if let Some(chunk) = client.current_chunk() {
            let last_biome = self.current_biome;
            let current_biome = chunk.meta().biome();

            if current_biome != last_biome {
                self.current_biome = current_biome;
                audio.fade_out(self.current_music, 1.0);
                match current_biome {
                    BiomeKind::Void => {
                        // Play no music
                    }
                    BiomeKind::Grassland => {
                        audio.play_music("voxygen.audio.soundtrack.regional.grasslands", 10.0, 1.0);
                    }
                    BiomeKind::Ocean => {}
                    BiomeKind::Mountain => {
                        audio.play_music("voxygen.audio.soundtrack.Mineral_Deposits", 10.0, 1.0);
                    }
                    BiomeKind::Snowlands => {
                        audio.play_music("voxygen.audio.soundtrack.Snowtop_mountain", 10.0, 1.0);
                    }
                    BiomeKind::Desert => {}
                    BiomeKind::Swamp => {}
                    BiomeKind::Forest => {}
                }
            }
        }
    }
}

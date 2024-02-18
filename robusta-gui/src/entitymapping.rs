use bevy::utils::hashbrown::HashMap;
use robusta_core::RobustaEntity;

use super::*;

#[derive(Resource, Default)]
pub struct EntityMapping {
    pub hash: HashMap<Entity, RobustaEntity>,
    z_layer_index: f32,
}

impl EntityMapping {
    pub fn new() -> EntityMapping {
        EntityMapping {
            hash: HashMap::new(),
            z_layer_index: 0.0,
        }
    }

    pub fn get(&self, entity: &Entity) -> Option<&RobustaEntity> {
        return self.hash.get(entity);
    }

    pub fn z_layer_add(&mut self) -> f32 {
        self.z_layer_index += 0.0001;

        self.z_layer_index
    }
}

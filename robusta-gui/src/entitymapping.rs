use bevy::utils::hashbrown::HashMap;
use robusta_core::RobustaEntity;

use super::*;

#[derive(Resource)]
pub struct EntityMapping {
    pub hash: HashMap<Entity, RobustaEntity>,
}

impl EntityMapping {
    pub fn new() -> EntityMapping {
        EntityMapping {
            hash: HashMap::new(),
        }
    }

    pub fn get(&self, entity: &Entity) -> Option<&RobustaEntity> {
        return self.hash.get(entity);
    }
}
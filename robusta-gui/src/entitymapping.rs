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

    pub fn hash(&self, entity: &Entity) -> &RobustaEntity {
        if let Some(a) = self.hash.get(entity) {
            return a;
        }
        return &RobustaEntity::None;
    }
}

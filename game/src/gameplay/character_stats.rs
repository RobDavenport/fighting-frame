use crate::db::CharacterDefinition;

pub struct CharacterStats {
    pub health: usize,
    pub boost_meter: usize,
    pub super_meter: usize,
}

impl CharacterStats {
    pub fn new(definition: &'static CharacterDefinition) -> Self {
        Self {
            health: definition.max_hp,
            boost_meter: 1000,
            super_meter: 0,
        }
    }
}

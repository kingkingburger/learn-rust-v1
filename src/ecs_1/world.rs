use crate::{component_storage::{Name, Position}, entity::{self, Entity}};

pub struct World {
    entities: Vec<Entity>,
    next_id: u32,
    pub position: Vec<Option<Position>>,
    names: Vec<Option<Name>>,
}

impl World {
    pub fn new() -> Self {
        World {
            entities: Vec::new(),
            position: Vec::new(),
            names: Vec::new(),
            next_id: 0,
        }
    }

    pub fn spawn(&mut self) -> Entity {
        let id = self.next_id;
        let new_entity = Entity(id);
        
        self.entities.push(new_entity);
        self.position.push(None);
        self.names.push(None);
        self.next_id = id + 1;

        new_entity
    }

    pub fn add_position(&mut self, entity: Entity, pos: Position) {
        let id = entity.0 as usize;

        self.position[id] = Some(pos);
    }
}
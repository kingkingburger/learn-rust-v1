struct World {
    entities: Vec<Entity>,
    next_id: u32,
    position: Vec<Option<Position>>,
    names: Vec<Option<Name>>,
}

impl World {
    fn new() -> Self {
        World {
            entities: Vec::new(),
            next_id: 0,
        }
    }

    fn spawn(&mut self) -> Entity {
        let id = self.next_id;
        let new_entity = Entity(id);
        
        self.entities.push(new_entity);

        self.next_id = id + 1;

        new_entity
    }
}
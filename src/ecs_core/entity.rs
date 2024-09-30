// entity.rs

pub type Entity = u32;

pub struct EntityManager {
    next_entity: Entity,
    recycled_entities: Vec<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_entity: 0,
            recycled_entities: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        if let Some(entity) = self.recycled_entities.pop() {
            entity
        } else {
            let entity = self.next_entity;
            self.next_entity += 1;
            entity
        }
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.recycled_entities.push(entity);
    }
}
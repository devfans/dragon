
pub struct Entity {
    pub id: u32,
    pub components: u32,
}

impl Entity {
}

pub struct EntityManager {
    coder: u32,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            coder: 1,
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let id = self.coder;
        self.coder += 1;
        Entity {
            id,
            components: 0,
        }
    }

    pub fn reset(&mut self) {
        self.coder = 1;
    }
}



use bon::Builder;
use common::{game::GameFrame, space::context::ClientGeoContext};
use uuid::Uuid;

#[derive(Builder)]
pub struct TaskContext {
    id: Uuid,
    start: GameFrame,
    end: GameFrame,
}

impl TaskContext {
    pub fn is_finished(&self, frame: GameFrame) -> bool {
        frame >= self.end
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn start(&self) -> GameFrame {
        self.start
    }

    pub fn end(&self) -> GameFrame {
        self.end
    }
}

#[derive(Builder, Clone)]
pub struct GeoContext {
    x: u64,
    y: u64,
}

impl GeoContext {
    pub fn xy(&self) -> (u64, u64) {
        (self.x, self.y)
    }

    pub fn set_xy(&mut self, value: (u64, u64)) {
        self.x = value.0;
        self.y = value.1;
    }
}

impl Into<ClientGeoContext> for GeoContext {
    fn into(self) -> ClientGeoContext {
        ClientGeoContext::new(self.x, self.y)
    }
}

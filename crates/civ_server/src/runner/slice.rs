use common::{game::slice::Slice, space::window::Window};

use crate::{runner::Runner, world::WorldItem};

pub struct GameSlice(Slice<Vec<Option<&WorldItem>>>);

impl std::ops::Deref for GameSlice {
    type Target = Slice<Vec<Option<&WorldItem>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Runner {
    pub fn slice(self: &Runner, window: &Window) -> GameSlice {
        let state = self.context.state();
        let world = self
            .context
            .world
            .read()
            .expect("Consider world as always readable");
        GameSlice(Slice::new(
            *window.start(),
            (window.end().x - window.start().x + 1) as u64,
            (window.end().y - window.start().y + 1) as u64,
            world.slice(window),
        ))
    }
}

use std::thread;

use uuid::Uuid;

use super::{CommandContext, CommandError, FOLLOW_INTERVAL};

pub fn cities(context: CommandContext) -> Result<(), CommandError> {
    let state = context
        .state
        .read()
        .expect("Consider state always accessible");

    for city in state.cities()? {
        println!("{}: {}", city.id(), city.name())
    }

    Ok(())
}

pub fn city(context: CommandContext, id: Uuid, follow: bool) -> Result<(), CommandError> {
    let state = context
        .state
        .read()
        .expect("Consider state always accessible");

    let mut follow_ = true;
    while follow_ && !context.context.stop_is_required() {
        if let Some(city) = state.cities()?.iter().find(|c| c.id() == id) {
            println!("id: {}", city.id());
            println!("name: {}", city.name());
            println!("xy: {:?}", city.geo().point());
        }
        follow_ = follow;

        if follow_ {
            thread::sleep(FOLLOW_INTERVAL);
        }
    }

    Ok(())
}

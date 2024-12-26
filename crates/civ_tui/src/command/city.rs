use uuid::Uuid;

use super::{CommandContext, CommandError};

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

pub fn city(context: CommandContext, id: Uuid) -> Result<(), CommandError> {
    let state = context
        .state
        .read()
        .expect("Consider state always accessible");

    if let Some(city) = state.cities()?.iter().find(|c| c.id() == id) {
        println!("id: {}", city.id());
        println!("name: {}", city.name());
        println!("xy: {:?}", city.geo().point());
    }

    Ok(())
}

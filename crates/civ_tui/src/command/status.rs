use super::CommandContext;

pub fn status(context: CommandContext) {
    let state = context
        .state
        .read()
        .expect("Assume state is always accessible");

    let window_str = state
        .window()
        .map(|w| w.to_string())
        .unwrap_or("n/a".to_string());
    let tiles_str = state
        .world()
        .map(|w| w.tiles().len().to_string())
        .unwrap_or("n/a".to_string());
    let flag_str = state
        .flag()
        .map(|f| f.to_string())
        .unwrap_or("n/a".to_string());

    println!("client_id: {}", state.client_id());
    println!("connected: {}", context.context.is_connected());
    println!("flag: {}", flag_str);
    println!("errors: {}", state.errors().len());
    println!("window: {} ({})", window_str, tiles_str);
    println!(
        "cities: {}",
        state
            .cities()
            .as_ref()
            .map(|cities| cities.len().to_string())
            .unwrap_or("n/a".to_string())
    );
    println!(
        "units: {}",
        state
            .units()
            .as_ref()
            .map(|units| units.len().to_string())
            .unwrap_or("n/a".to_string())
    );
}

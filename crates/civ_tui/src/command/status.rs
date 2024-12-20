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

    println!("client_id: {}", state.client_id());
    println!("connected: {}", state.connected());
    println!("errors: {}", state.errors().len());
    println!("window: {}", window_str);
    println!(
        "cities: {}",
        state
            .cities()
            .as_ref()
            .and_then(|cities| Some(cities.len().to_string()))
            .unwrap_or("n/a".to_string())
    );
    println!(
        "units: {}",
        state
            .units()
            .as_ref()
            .and_then(|units| Some(units.len().to_string()))
            .unwrap_or("n/a".to_string())
    );
}

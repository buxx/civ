use super::CommandContext;

pub fn errors(context: CommandContext) {
    let mut state = context
        .state
        .write()
        .expect("Assume state is always accessible");

    for error in state.errors() {
        println!("{}", error);
    }

    state.clear_error();
}

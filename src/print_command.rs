use crate::command::{Command, CommandState, InterruptionBehavior};

#[derive(std::fmt::Debug)]
pub struct PrintCommand {
    to_print: String,
    state: CommandState,
}

impl PrintCommand {
    pub fn new(to_print: String) -> Self {
        Self {
            to_print,
            state: CommandState {
                interruption_behavior: InterruptionBehavior::KCancelSelf,
                requirements: vec![],
            },
        }
    }
}

impl Command for PrintCommand {
    fn initialize(&mut self) {
        println!("{}", self.to_print)
    }

    fn get_state(&self) -> &crate::command::CommandState {
        return &self.state;
    }
}

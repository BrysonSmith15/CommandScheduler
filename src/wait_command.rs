use crate::command;

#[allow(dead_code)]
#[derive(std::fmt::Debug)]
pub struct WaitCommand {
    pub state: command::CommandState,
    start_time: std::time::Instant,
    wait_duration: std::time::Duration,
}

impl WaitCommand {
    pub fn new(wait_duration: std::time::Duration) -> Self {
        Self {
            state: command::CommandState {
                interruption_behavior: command::InterruptionBehavior::KCancelIncoming,
                requirements: vec![],
            },
            start_time: std::time::Instant::now(),
            wait_duration,
        }
    }
}

impl command::Command for WaitCommand {
    fn initialize(&mut self) {
        println!("Starting wait for {:#?}", self.wait_duration);
        self.start_time = std::time::Instant::now();
    }

    fn is_finished(&mut self) -> bool {
        return self.start_time.elapsed() >= self.wait_duration;
    }

    fn end(&mut self, interrupted: bool) {
        if interrupted {
            println!("Canceling MyCommand");
        } else {
            println!("MyCommand ending naturally");
        }
    }

    fn get_state(&self) -> &command::CommandState {
        return &self.state;
    }
}

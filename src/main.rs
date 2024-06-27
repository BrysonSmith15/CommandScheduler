use std::any::Any;

mod command;
mod subsystem;

mod command_scheduler;

mod print_command;
mod wait_command;

mod sequential_command_group;

#[allow(dead_code)]
#[derive(std::fmt::Debug)]
struct MySubsystem {}

impl subsystem::Subsystem for MySubsystem {
    fn is_eq(&self, other: &'static dyn subsystem::Subsystem) -> bool {
        self.type_id() == other.type_id()
    }
}

fn main() {
    let mut scheduler = command_scheduler::CommandScheduler::from_default();
    let wait_cmd = wait_command::WaitCommand::new(std::time::Duration::from_secs(5));
    let printcmd = print_command::PrintCommand::new(String::from("Best command program ever"));
    scheduler.schedule(Box::new(printcmd)).unwrap();
    scheduler.schedule(Box::new(wait_cmd)).unwrap();
    loop {
        scheduler.run();
    }
}

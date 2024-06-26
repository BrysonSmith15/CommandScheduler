use command::CommandBase;

mod command;
mod subsystem;

#[allow(dead_code)]
struct MyCommand {
    state: command::CommandState,
    start_time: std::time::Instant,
}

#[allow(dead_code)]
#[derive(std::fmt::Debug)]
struct MySubsystem {}

impl subsystem::Subsystem for MySubsystem {}

impl MyCommand {
    fn new() -> Self {
        Self {
            state: command::CommandState {
                interruption_behavior: command::InterruptionBehavior::KCancelIncoming,
                requirements: vec![],
            },
            start_time: std::time::Instant::now(),
        }
    }
}

impl CommandBase for MyCommand {
    fn add_requirement(&mut self, requirement: &'static dyn subsystem::Subsystem) {
        self.state.requirements.push(requirement);
    }

    fn initialize(&mut self) {
        self.start_time = std::time::Instant::now();
    }

    fn execute(&mut self) {
        println!("{:#?}", self.start_time.elapsed())
    }

    fn is_finished(&mut self) -> bool {
        return self.start_time.elapsed() >= std::time::Duration::from_secs_f32(1.5);
    }

    fn end(&mut self, interrupted: bool) {
        if interrupted {
            println!("Canceling MyCommand");
        } else {
            println!("MyCommand ending naturally");
        }
    }

    fn get_requirements(&self) -> &Vec<&dyn subsystem::Subsystem> {
        return &self.state.requirements;
    }
}

fn main() {
    let mut cmd = MyCommand::new();
    cmd.add_requirements(vec![&MySubsystem {}]);
    println!("{:?}", cmd.get_requirements());
    loop {
        cmd.execute();
        if cmd.is_finished() {
            cmd.end(true);
            break;
        }
    }
    println!("Hello, world!");
}

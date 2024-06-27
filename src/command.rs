use crate::subsystem;

#[allow(dead_code)]
#[derive(std::fmt::Debug, PartialEq)]
pub enum InterruptionBehavior {
    KCancelIncoming,
    KCancelSelf,
}

#[allow(dead_code)]
#[derive(std::fmt::Debug)]
pub struct CommandState {
    pub interruption_behavior: InterruptionBehavior,
    pub requirements: Vec<&'static dyn subsystem::Subsystem>,
}

#[allow(dead_code)]
pub trait CommandBase {
    fn add_requirement(&mut self, requirement: &'static dyn subsystem::Subsystem);
    fn add_requirements(&mut self, requirements: Vec<&'static dyn subsystem::Subsystem>) {
        for requirement in requirements.iter() {
            self.add_requirement(*requirement);
        }
    }
    fn get_requirements(&self) -> &Vec<&dyn subsystem::Subsystem>;
}

impl CommandBase for CommandState {
    fn add_requirement(&mut self, requirement: &'static dyn subsystem::Subsystem) {
        self.requirements.push(requirement);
    }

    fn get_requirements(&self) -> &Vec<&dyn subsystem::Subsystem> {
        return &self.requirements;
    }
}

#[allow(dead_code)]
pub trait Command: std::fmt::Debug {
    fn initialize(&mut self) {}
    fn execute(&mut self) {}
    fn is_finished(&mut self) -> bool {
        true
    }
    #[allow(unused_variables)]
    fn end(&mut self, interrupted: bool) {}

    fn get_state(&self) -> &CommandState;
}

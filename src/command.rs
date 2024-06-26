use crate::subsystem;

#[allow(dead_code)]
pub enum InterruptionBehavior {
    KCancelIncoming,
    KCancelSelf,
}

#[allow(dead_code)]
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

    fn initialize(&mut self);
    fn execute(&mut self);
    fn is_finished(&mut self) -> bool;
    fn end(&mut self, interrupted: bool);
}

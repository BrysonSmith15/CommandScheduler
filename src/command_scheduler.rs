use std::any::{type_name_of_val, Any};

use crate::command::{self, CommandBase};

pub struct CommandScheduler {
    loop_rate: std::time::Duration,
    last_run: std::time::Instant,
    commands: Vec<Box<dyn command::Command>>,
    to_schedule: Vec<Box<dyn command::Command>>,
}

impl CommandScheduler {
    pub fn new(loop_rate: std::time::Duration) -> Self {
        CommandScheduler {
            loop_rate,
            last_run: std::time::Instant::now(),
            commands: vec![],
            to_schedule: vec![],
        }
    }

    pub fn from_default() -> Self {
        Self::new(std::time::Duration::from_millis(20))
    }

    pub fn is_scheduled(&self, cmd: Box<dyn command::Command>) -> bool {
        self.commands
            .iter()
            .fold(false, |acc, curr| acc || Self::command_eq(&cmd, curr))
    }

    pub fn schedule(&mut self, new_command: Box<dyn command::Command>) -> Result<(), ()> {
        self.commands.iter().fold(false, |acc, curr| {
            //*curr.get_state().get_requirements().iter().fold(init, f)
            acc
        });
        self.to_schedule.push(new_command);
        Ok(())
    }

    pub fn run(&mut self) {
        if self.last_run.elapsed() >= self.loop_rate {
            while let Some(mut cmd) = self.to_schedule.pop() {
                cmd.initialize();
                self.commands.push(cmd);
            }

            let mut to_remove: Vec<usize> = vec![];
            for i in 0..self.commands.len() {
                let cmd = &mut self.commands[i];
                cmd.execute();
                let finished = cmd.is_finished();
                if finished {
                    cmd.end(false);
                    to_remove.push(i);
                }
            }
            for idx in to_remove.iter() {
                self.commands.remove(*idx);
            }
        }
    }

    fn command_eq(c1: &Box<dyn command::Command>, c2: &Box<dyn command::Command>) -> bool {
        return c1.type_id() == c2.type_id();
    }
}

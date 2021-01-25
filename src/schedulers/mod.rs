mod thread_scheduler;

use crate::tasks::{Task, TaskPointer};

pub enum SchedulerType {
    Thread
}

pub trait Scheduler {
    fn add_task(&mut self, task: TaskPointer);
}

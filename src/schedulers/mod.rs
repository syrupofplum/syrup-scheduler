mod thread_scheduler;

use crate::tasks::Task;

pub enum SchedulerType {
    Thread
}

pub trait Scheduler {
    fn add_task(&mut self, task: Box<dyn Task>);
}

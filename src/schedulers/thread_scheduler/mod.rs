use crate::schedulers::Scheduler;
use crate::tasks::{Task, TaskBasket};

pub struct ThreadScheduler {
    task_basket: dyn TaskBasket
}

impl Scheduler for ThreadScheduler {
    fn add_task(&mut self, task: Box<dyn Task>) {

    }
}

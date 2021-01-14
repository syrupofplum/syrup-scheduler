use crate::schedulers::Scheduler;
use crate::triggers::{Trigger, IntervalTrigger};
use crate::tasks::{Task, TaskBasket, HeapTaskBasket};

pub struct ThreadScheduler {
    task_basket: Box<dyn TaskBasket>,
    trigger: Box<dyn Trigger>
}

impl ThreadScheduler {
    fn new() -> Self {
        Self {
            task_basket: Box::new(HeapTaskBasket::new()),
            trigger: Box::new(IntervalTrigger::new())
        }
    }
}

impl Scheduler for ThreadScheduler {
    fn add_task(&mut self, task: Box<dyn Task>) {
        self.task_basket.add_task_sync(task);
        self.trigger.refresh();
    }
}

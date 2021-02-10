use crate::schedulers::Scheduler;
use crate::triggers::{Trigger, IntervalTrigger, TriggerManager};
use crate::tasks::{Task, TaskBasket, HeapTaskBasket, TaskPointer};

pub struct ThreadScheduler {
    task_basket: Box<dyn TaskBasket>,
    trigger_manager: TriggerManager,
}

impl ThreadScheduler {
    fn new() -> Self {
        Self {
            task_basket: Box::new(HeapTaskBasket::new()),
            trigger_manager: TriggerManager::new()
        }
    }
}

impl Scheduler for ThreadScheduler {
    fn add_task(&mut self, task: TaskPointer) {
        self.task_basket.add_task(task);
        self.trigger_manager.refresh();
    }
}

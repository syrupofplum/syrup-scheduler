use std::collections::{BinaryHeap, HashMap};
use crate::tasks::{TaskBasket, Task, TaskPointer};
use crate::errors::{ErrorBundle, ErrorBundleType};

pub struct HeapTaskBasket {
    storage: BinaryHeap<TaskPointer>
}

impl HeapTaskBasket {
    pub fn new() -> Self {
        Self {
            storage: BinaryHeap::new()
        }
    }
}

impl TaskBasket for HeapTaskBasket {
    fn add_task(&mut self, task: TaskPointer) -> Result<(), ErrorBundle> {
        self.storage.push(task);
        Ok(())
    }

    fn remove_task(&mut self) -> Result<TaskPointer, ErrorBundle> {
        let cur_task = self.storage.pop().ok_or(ErrorBundle::new().err_type(ErrorBundleType::TaskBasketEmpty).err_msg("basket empty."))?;
        Ok(cur_task)
    }
}

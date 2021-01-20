use std::collections::{BinaryHeap, HashMap};
use crate::tasks::{TaskBasket, Task};
use crate::errors::{ErrorBundle, ErrorBundleType};

pub struct HeapTaskBasket {
    storage: BinaryHeap<Box<dyn Task>>
}

impl HeapTaskBasket {
    pub fn new() -> Self {
        Self {
            storage: BinaryHeap::new()
        }
    }
}

impl TaskBasket for HeapTaskBasket {
    fn add_task_sync(&mut self, task: Box<dyn Task>) -> Result<(), ErrorBundle> {
        self.storage.push(task);
        Ok(())
    }

    fn remove_task_sync(&mut self) -> Result<Box<dyn Task>, ErrorBundle> {
        let cur_task = self.storage.pop().ok_or(ErrorBundle::new().err_type(ErrorBundleType::TaskBasketEmpty).err_msg("basket empty."))?;
        Ok(cur_task)
    }
}

use std::collections::{BinaryHeap, HashMap};
use crate::tasks::{TaskBasket, Task};
use crate::errors::ErrorBundle;

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
        Ok(())
    }
}

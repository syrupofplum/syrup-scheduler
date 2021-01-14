use std::collections::BinaryHeap;
use std::collections::LinkedList;
use crate::tasks::{TaskBasket, Task};
use crate::errors::ErrorBundle;

pub struct HeapTaskBasket {
}

impl HeapTaskBasket {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl TaskBasket for HeapTaskBasket {
    fn add_task_sync(&mut self, task: Box<dyn Task>) -> Result<(), ErrorBundle> {
        Ok(())
    }
}

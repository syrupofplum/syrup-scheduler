use crate::errors::ErrorBundle;
mod task_basket;

pub enum TaskState {
    Ready,
    Running,
    Suspended,
    Stopped
}

pub trait Task {
    fn get_state(&self) -> Result<TaskState, ErrorBundle>;
}

pub trait TaskBasket {
    fn add_task_sync(&mut self, task: Box<dyn Task>) -> Result<(), ErrorBundle>;
}

pub use task_basket::HeapTaskBasket;

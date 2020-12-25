use crate::errors::ErrorBundle;

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
    fn add_task_sync(&mut self) -> Result<(), ErrorBundle>;
}

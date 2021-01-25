use crate::errors::ErrorBundle;
mod task_basket;

pub enum TaskState {
    Ready,
    Running,
    Suspended,
    Stopped
}

pub trait TaskBaseInfo {
    fn get_id(&self) -> Result<&str, ErrorBundle>;
}

pub trait TaskTriggerInfo {
    fn get_state(&self) -> Result<TaskState, ErrorBundle>;
    fn get_next_exec_time(&self) -> Result<chrono::Duration, ErrorBundle>;
}

pub trait TaskRunStatInfo {
}

pub trait Task : TaskBaseInfo + TaskTriggerInfo + TaskRunStatInfo {
}

pub struct TaskPointer {
    pointer: Box<dyn Task>
}

impl TaskBaseInfo for TaskPointer {
    fn get_id(&self) -> Result<&str, ErrorBundle> {
        self.pointer.get_id()
    }
}

impl TaskTriggerInfo for TaskPointer {
    fn get_state(&self) -> Result<TaskState, ErrorBundle> {
        self.pointer.get_state()
    }

    fn get_next_exec_time(&self) -> Result<Duration, ErrorBundle> {
        self.pointer.get_next_exec_time()
    }
}

impl PartialEq for TaskPointer {
    fn eq(&self, other: &Self) -> bool {
        if self.get_id().is_err() || other.get_id().is_err() {
            return false;
        }
        self.get_id().unwrap() == other.get_id().unwrap()
    }
}

impl Eq for TaskPointer {
}

impl PartialOrd for TaskPointer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.get_next_exec_time().is_err() || other.get_next_exec_time().is_err() {
            return None;
        }
        self.get_next_exec_time().unwrap().partial_cmp(&other.get_next_exec_time().unwrap())
    }
}

impl Ord for TaskPointer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.get_next_exec_time().is_err() || other.get_next_exec_time().is_err() {
            return std::cmp::Ordering::Equal;
        }
        self.get_next_exec_time().unwrap().cmp(&other.get_next_exec_time().unwrap())
    }
}

pub struct TaskWeakPointer {
    pointer: std::rc::Weak<dyn Task>
}

impl TaskWeakPointer {
    pub fn upgrade(&self) -> Option<std::rc::Rc<dyn Task>> {
        self.upgrade()
    }
}

impl PartialEq for TaskWeakPointer {
    fn eq(&self, other: &Self) -> bool {
        if self.upgrade().is_none() || other.upgrade().is_none() {
            return false;
        }
        if self.upgrade().unwrap().get_id().is_err() || other.upgrade().unwrap().get_id().is_err() {
            return false;
        }
        self.upgrade().unwrap().get_id().unwrap() == other.upgrade().unwrap().get_id().unwrap()
    }
}

impl Eq for TaskWeakPointer {
}

impl PartialOrd for TaskWeakPointer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.upgrade().is_none() || other.upgrade().is_none() {
            return None;
        }
        if self.upgrade().unwrap().get_next_exec_time().is_err() || other.upgrade().unwrap().get_next_exec_time().is_err() {
            return None;
        }
        self.upgrade().unwrap().get_next_exec_time().unwrap().partial_cmp(&other.upgrade().unwrap().get_next_exec_time().unwrap())
    }
}

impl Ord for TaskWeakPointer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.upgrade().is_none() || other.upgrade().is_none() {
            return std::cmp::Ordering::Equal;
        }
        if self.upgrade().unwrap().get_next_exec_time().is_err() || other.upgrade().unwrap().get_next_exec_time().is_err() {
            return std::cmp::Ordering::Equal;
        }
        self.upgrade().unwrap().get_next_exec_time().unwrap().cmp(&other.upgrade().unwrap().get_next_exec_time().unwrap())
    }
}

pub trait TaskBasket {
    fn add_task(&mut self, task: TaskPointer) -> Result<(), ErrorBundle>;
    fn remove_task(&mut self) -> Result<TaskPointer, ErrorBundle>;
}

pub use task_basket::HeapTaskBasket;
use chrono::Duration;

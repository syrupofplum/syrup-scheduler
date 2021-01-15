use crate::errors::ErrorBundle;
mod task_basket;

pub enum TaskState {
    Ready,
    Running,
    Suspended,
    Stopped
}

pub trait Task {
    fn get_id(&self) -> Result<&str, ErrorBundle>;
    fn get_state(&self) -> Result<TaskState, ErrorBundle>;
    fn get_next_exec_time(&self) -> Result<chrono::Duration, ErrorBundle>;
}

impl PartialEq for Box<dyn Task> {
    fn eq(&self, other: &Self) -> bool {
        if self.get_id().is_err() || other.get_id().is_err() {
            return false;
        }
        self.get_id().unwrap() == other.get_id().unwrap()
    }
}

impl Eq for Box<dyn Task> {
}

impl PartialOrd for Box<dyn Task> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.get_next_exec_time().is_err() || other.get_next_exec_time().is_err() {
            return None;
        }
        self.get_next_exec_time().unwrap().partial_cmp(&other.get_next_exec_time().unwrap())
    }
}

impl Ord for Box<dyn Task> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.get_next_exec_time().is_err() || other.get_next_exec_time().is_err() {
            return std::cmp::Ordering::Equal;
        }
        self.get_next_exec_time().unwrap().cmp(&other.get_next_exec_time().unwrap())
    }
}

// impl PartialEq for std::rc::Weak<dyn Task> {
//     fn eq(&self, other: &Self) -> bool {
//         if self.upgrade().is_none() || other.upgrade().is_none() {
//             return false;
//         }
//         if self.upgrade().unwrap().get_id().is_err() || other.upgrade().unwrap().get_id().is_err() {
//             return false;
//         }
//         self.upgrade().unwrap().get_id().unwrap() == other.upgrade().unwrap().get_id().unwrap()
//     }
// }
//
// impl Eq for std::rc::Weak<dyn Task> {
// }
//
// impl PartialOrd for std::rc::Weak<dyn Task> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         if self.upgrade().is_none() || other.upgrade().is_none() {
//             return None;
//         }
//         if self.upgrade().unwrap().get_next_exec_time().is_err() || other.upgrade().unwrap().get_next_exec_time().is_err() {
//             return None;
//         }
//         self.upgrade().unwrap().get_next_exec_time().unwrap().partial_cmp(&other.upgrade().unwrap().get_next_exec_time().unwrap())
//     }
// }
//
// impl Ord for std::rc::Weak<dyn Task> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         if self.upgrade().is_none() || other.upgrade().is_none() {
//             return std::cmp::Ordering::Equal;
//         }
//         if self.upgrade().unwrap().get_next_exec_time().is_err() || other.upgrade().unwrap().get_next_exec_time().is_err() {
//             return std::cmp::Ordering::Equal;
//         }
//         self.upgrade().unwrap().get_next_exec_time().unwrap().cmp(&other.upgrade().unwrap().get_next_exec_time().unwrap())
//     }
// }

pub trait TaskBasket {
    fn add_task_sync(&mut self, task: Box<dyn Task>) -> Result<(), ErrorBundle>;
}

pub use task_basket::HeapTaskBasket;

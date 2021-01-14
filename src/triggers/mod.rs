use crate::errors::ErrorBundle;

mod interval_trigger;

pub enum TriggerRepeatType {
    Cron,
    Interval,
    Random
}

pub struct TriggerRepeatInfo {
    total: u64
}

pub struct TriggerRange {
    start_time: std::time::Instant,
    end_time: std::time::Instant
}

pub struct TriggerRunningInfo {
    first_run_time: std::time::Instant,
    next_run_time: Option<std::time::Instant>,
    create_time: std::time::Instant,
    counter: u64
}

pub trait Trigger {
    fn refresh(&mut self) -> Result<(), ErrorBundle>;
    fn shoot(&mut self) -> Result<(), ErrorBundle>;
}

pub use interval_trigger::IntervalTrigger;

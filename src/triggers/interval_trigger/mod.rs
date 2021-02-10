use std::thread;

use crate::triggers::{TriggerRepeatType, TriggerRepeatInfo, TriggerRange, TriggerRunningInfo, Trigger};
use crate::errors::ErrorBundle;

impl TriggerRepeatInfo {
    pub fn new() -> Self {
        Self {
            total: 0
        }
    }
}

impl TriggerRange {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            end_time: std::time::Instant::now()
        }
    }
}

impl TriggerRunningInfo {
    pub fn new() -> Self {
        Self {
            first_run_time: std::time::Instant::now(),
            next_run_time: None,
            create_time: std::time::Instant::now(),
            counter: 0
        }
    }
}

pub struct IntervalTrigger {
    repeat_type: TriggerRepeatType,
    repeat_times: TriggerRepeatInfo,
    range: TriggerRange,
    running_info: TriggerRunningInfo
}

impl IntervalTrigger {
    pub fn new() -> Self {
        Self {
            repeat_type: TriggerRepeatType::Cron,
            repeat_times: TriggerRepeatInfo::new(),
            range: TriggerRange::new(),
            running_info: TriggerRunningInfo::new()
        }
    }
}

impl Trigger for IntervalTrigger {
    fn shoot(&mut self) -> Result<(), ErrorBundle> {
        thread::spawn(|| {
            
        });
        Result::Ok(())
    }
}

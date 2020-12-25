use crate::triggers::{TriggerRepeatType, TriggerRepeatTimes, TriggerRange, TriggerRunningInfo, Trigger};
use crate::errors::ErrorBundle;

pub struct IntervalTrigger {
    repeat_type: TriggerRepeatType,
    repeat_times: TriggerRepeatTimes,
    range: TriggerRange,
    running_info: TriggerRunningInfo
}

impl Trigger for IntervalTrigger {
    fn refresh(&mut self) -> Result<(), ErrorBundle> {
        Result::Ok(())
    }

    fn shoot(&mut self) -> Result<(), ErrorBundle> {
        Result::Ok(())
    }
}

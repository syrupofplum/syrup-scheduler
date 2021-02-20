use crate::errors::ErrorBundle;
use std::sync::mpsc::{Sender, Receiver, channel};

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
    fn shoot(&mut self) -> Result<(), ErrorBundle>;
}

pub use interval_trigger::IntervalTrigger;

#[derive(Debug)]
pub enum TriggerManagerSignal {
    Update
}

struct TriggerManagerSignalSender {
    tx: Sender<TriggerManagerSignal>
}

impl TriggerManagerSignalSender {
    fn new(tx: Sender<TriggerManagerSignal>) -> Self {
        Self {
            tx
        }
    }
    
    fn get_sender(self) -> Sender<TriggerManagerSignal> {
        self.tx.clone()
    }
}

struct TriggerManagerSignalReceiver {
    rx: Receiver<TriggerManagerSignal>
}

impl TriggerManagerSignalReceiver {
    fn new(rx: Receiver<TriggerManagerSignal>) -> Self {
        Self {
            rx
        }
    }
    
    fn get_receiver(self) -> Receiver<TriggerManagerSignal> {
        self.rx
    }
}

pub struct TriggerManager {
    sender: Option<TriggerManagerSignalSender>
}

impl TriggerManager {
    pub fn new() -> Self {
        Self {
            sender: None
        }
    }

    pub fn refresh(&mut self) -> Result<(), ErrorBundle> {
        Ok(())
    }

    pub fn run(&mut self) {
        let (tx, rx) = channel::<TriggerManagerSignal>();
        self.sender = Some(TriggerManagerSignalSender::new(tx));

        let thread_loop = std::thread::spawn(move || {
            loop {
                let signal = rx.recv();
            }
        });
    }
}

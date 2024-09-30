// temporal.rs

use std::collections::BinaryHeap; //for event que
use std::cmp::Ordering;
use web_sys::Performance;
use tracing::{info, error};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MixedRadixTime {
    frame_count: u64,
    ticks: u64,
    sub_ticks: u32,
    milliseconds: u32,
}

impl MixedRadixTime {
    pub fn new() -> Self {
        Self { frame_count: 0, ticks: 0, sub_ticks: 0, milliseconds: 0 }
    }

    pub fn advance(&mut self, delta_ms: u32, sub_ticks_per_tick: u32, ms_per_sub_tick: u32) {
        self.frame_count += 1;
        self.milliseconds += delta_ms;
        let new_sub_ticks = self.milliseconds / ms_per_sub_tick;
        self.milliseconds %= ms_per_sub_tick;
        self.sub_ticks += new_sub_ticks;
        let new_ticks = self.sub_ticks / sub_ticks_per_tick;
        self.sub_ticks %= sub_ticks_per_tick;
        self.ticks += new_ticks as u64;
    }
}

struct TemporalEvent {
    time: MixedRadixTime,
    id: u64,
    payload: Box<dyn Fn()>,
}

impl PartialEq for TemporalEvent { 
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.id == other.id
    }
}

impl Eq for TemporalEvent {}

impl PartialOrd for TemporalEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TemporalEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // (min-heap behavior)
        other.time.cmp(&self.time)
            .then_with(|| other.id.cmp(&self.id))
    }
}

pub struct AdvancedTime {
    mixed_time: MixedRadixTime,
    time_scale: f32,
    paused: bool,
    performance: Performance,
    last_timestamp: f64,
    event_queue: BinaryHeap<TemporalEvent>,
    next_event_id: u64,
    sub_ticks_per_tick: u32,
    ms_per_sub_tick: u32,
    last_delta_ms: u32,
}

impl AdvancedTime {
    pub fn new(sub_ticks_per_tick: u32, ms_per_sub_tick: u32) -> Self {
        let performance = web_sys::window()
            .expect("no global accessible window exists")
            .performance()
            .expect("window should have performance");

        Self {
            mixed_time: MixedRadixTime::new(),
            time_scale: 1.0,
            paused: false,
            performance,
            last_timestamp: 0.0,
            event_queue: BinaryHeap::new(),
            next_event_id: 0,
            sub_ticks_per_tick,
            ms_per_sub_tick,
            last_delta_ms: 0,
        } 
    }

    pub fn update(&mut self) {
        if self.paused {
            return;
        }

        let current_timestamp = self.performance.now();
        if self.last_timestamp == 0.0 {
            self.last_timestamp = current_timestamp;
            return;
        }

        let delta_ms = ((current_timestamp - self.last_timestamp) * self.time_scale as f64) as u32;
        self.last_timestamp = current_timestamp;
        self.last_delta_ms = delta_ms;
        self.mixed_time.advance(delta_ms, self.sub_ticks_per_tick, self.ms_per_sub_tick);

        self.process_events();
    }

    pub fn schedule_event<F>(&mut self, delay: MixedRadixTime, payload: F) -> u64
    where
        F: Fn() + 'static,
    {
        let event_time = MixedRadixTime {
            frame_count: self.mixed_time.frame_count + delay.frame_count,
            ticks: self.mixed_time.ticks + delay.ticks,
            sub_ticks: self.mixed_time.sub_ticks + delay.sub_ticks,
            milliseconds: self.mixed_time.milliseconds + delay.milliseconds,
        };
    
        let id = self.next_event_id;
        self.next_event_id += 1;
    
        self.event_queue.push(TemporalEvent {
            time: event_time,
            id,
            payload: Box::new(payload),
        });
    
        id    
    }

    pub fn get_delta_time(&self) -> u32 {
        self.last_delta_ms
    }

fn process_events(&mut self) {
    while let Some(event) = self.event_queue.peek() {
        if event.time.frame_count > self.mixed_time.frame_count {
            break;
        }
        if event.time.frame_count == self.mixed_time.frame_count
            && (event.time.ticks > self.mixed_time.ticks
                || (event.time.ticks == self.mixed_time.ticks
                    && (event.time.sub_ticks > self.mixed_time.sub_ticks
                        || (event.time.sub_ticks == self.mixed_time.sub_ticks
                            && event.time.milliseconds > self.mixed_time.milliseconds))))
        {
            break;
        }
        
        let event = self.event_queue.pop().unwrap();
        (event.payload)();
    }
}

    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.0);
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
        self.last_timestamp = self.performance.now();
    }

    pub fn get_interpolation_factor(&self) -> f32 {
        let sub_ticks_fraction = self.mixed_time.sub_ticks as f32 / self.sub_ticks_per_tick as f32;
        let milliseconds_fraction = self.mixed_time.milliseconds as f32 / (self.sub_ticks_per_tick * self.ms_per_sub_tick) as f32;
        
        sub_ticks_fraction + milliseconds_fraction
    }
}


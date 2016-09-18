use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct ScheduleEntry<T> {
    value: T,
    abs_time: u64,
    rel_time: u64,
    seq: u64,
}

impl<T> ScheduleEntry<T> {
    fn new(value: T, abs_time: u64, rel_time: u64, seq: u64) -> Self {
        ScheduleEntry {
            value: value,
            abs_time: abs_time,
            rel_time: rel_time,
            seq: seq,
        }
    }
}

impl<T> Ord for ScheduleEntry<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let abs_time_ord = other.abs_time.cmp(&self.abs_time);
        if abs_time_ord == Ordering::Equal {
            other.seq.cmp(&self.seq)
        } else {
            abs_time_ord
        }
    }
}

impl<T> PartialOrd for ScheduleEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for ScheduleEntry<T> {
    fn eq(&self, other: &Self) -> bool {
        self.abs_time == other.abs_time && self.seq == other.seq
    }
}

impl<T> Eq for ScheduleEntry<T> {}

pub struct Schedule<T> {
    heap: BinaryHeap<ScheduleEntry<T>>,
    abs_time: u64,
    seq: u64,
}

pub struct ScheduleEvent<T> {
    pub event: T,
    pub time_delta: u64,
    pub time_queued: u64,
}

impl<T> ScheduleEvent<T> {
    fn new(event: T, time_delta: u64, time_queued: u64) -> Self {
        ScheduleEvent {
            event: event,
            time_delta: time_delta,
            time_queued: time_queued,
        }
    }
}

impl<T> Schedule<T> {
    pub fn new() -> Self {
        Schedule {
            heap: BinaryHeap::new(),
            abs_time: 0,
            seq: 0,
        }
    }

    pub fn insert(&mut self, value: T, rel_time: u64) {
        let entry = ScheduleEntry::new(value, self.abs_time + rel_time, rel_time, self.seq);
        self.heap.push(entry);
        self.seq += 1;
    }

    pub fn next(&mut self) -> Option<ScheduleEvent<T>> {
        self.heap.pop().map(|entry| {
            assert!(entry.abs_time >= self.abs_time, "{} < {}", entry.abs_time, self.abs_time);
            let time_delta = entry.abs_time - self.abs_time;
            self.abs_time = entry.abs_time;

            ScheduleEvent::new(entry.value, time_delta, entry.rel_time)
        })
    }

    pub fn reset(&mut self) {
        self.heap.clear();
        self.abs_time = 0;
        self.seq = 0;
    }

    pub fn time(&self) -> u64 {
        self.abs_time
    }
}

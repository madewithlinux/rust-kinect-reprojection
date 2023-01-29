use lazy_static::lazy_static;

// TODO: use once_cell instead?
lazy_static! {
    static ref PERFORMANCE_TICKS_PER_MS: i64 = {
        let mut freq = 0;
        unsafe { windows::Win32::System::Performance::QueryPerformanceFrequency(&mut freq) }
            .ok()
            .unwrap();
        freq / 1_000
    };
}

pub fn query_performance_counter_ms() -> i64 {
    let mut counter = 0;
    unsafe { windows::Win32::System::Performance::QueryPerformanceCounter(&mut counter) }
        .ok()
        .unwrap();
    counter / *PERFORMANCE_TICKS_PER_MS
}

#[derive(Default, Debug, Clone)]
pub struct DelayBuffer<T> {
    dequeue: std::collections::VecDeque<(i64, T)>,
}

impl<T> DelayBuffer<T> {
    pub fn push(&mut self, value: T) {
        self.push_for_timestamp(query_performance_counter_ms(), value)
    }
    pub fn push_for_timestamp(&mut self, timestamp: i64, value: T) {
        if let Some(back) = self.dequeue.back() {
            assert!(timestamp >= back.0);
        }
        self.dequeue.push_back((timestamp, value))
    }

    pub fn back(&self) -> Option<&T> {
        self.dequeue.back().map(|(_, value)| value)
    }

    pub fn front(&self) -> Option<&T> {
        self.dequeue.front().map(|(_, value)| value)
    }

    /// pops and returns the newest element in the buffer that is at least `delay` milliseconds old
    pub fn pop_for_delay(&mut self, delay: i64) -> Option<T> {
        let now = query_performance_counter_ms();
        let max_timestamp = now - delay;
        loop {
            match (self.dequeue.get(0), self.dequeue.get(1)) {
                (None, _) => return None,

                (Some((t0, _)), _) if *t0 >= max_timestamp => {
                    // no elements are ready yet
                    return None;
                }
                (Some((t0, _)), Some((t1, _))) if *t0 < max_timestamp && *t1 < max_timestamp => {
                    // both are ready, so we pop the first and try again
                    self.dequeue.pop_front();
                    continue;
                }
                (Some((t0, _)), None) if *t0 < max_timestamp => {
                    // there is only one element, and it is ready
                    return Some(self.dequeue.pop_front().unwrap().1);
                }
                (Some((t0, _)), Some((t1, _))) if *t0 < max_timestamp && *t1 >= max_timestamp => {
                    // the first one is therefore the only one that is less than `delay` old
                    return Some(self.dequeue.pop_front().unwrap().1);
                }
                // TODO: is this really unreachable?
                (Some(_), _) => unreachable!(),
            };
        }
    }
}

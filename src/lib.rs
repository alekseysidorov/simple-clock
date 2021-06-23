#![cfg_attr(not(test), no_std)]

#[cfg(test)]
mod tests;

/// Provides an abstraction for hardware-specific clocks with the microsecond precision.
pub trait SimpleClock {
    /// Returns an instant time in the microseconds.
    fn now_us(&self) -> u64;
}

/// Provides an easy way to calculate elapsed times.
///
/// This timer is usually used to compute how much time is elapsed between two events
/// or to determine the event's deadline in case of polling.
pub struct ElapsedTimer<'a, T> {
    clock: &'a T,
    now: u64,
}

impl<'a, T: SimpleClock> ElapsedTimer<'a, T> {
    /// Creates a new elapsed timer instance backed by the specified clock implementation.
    pub fn new(clock: &'a T) -> Self {
        Self {
            clock,
            now: clock.now_us(),
        }
    }

    /// Restarts the timer and returns the number of microseconds
    /// elapsed since this timer was started.
    pub fn restart(&mut self) -> u64 {
        let elapsed = self.elapsed();
        self.now = self.clock.now_us();
        elapsed
    }

    /// Returns the number of microseconds elapsed since this timer was started.
    pub fn elapsed(&self) -> u64 {
        self.clock.now_us().saturating_sub(self.now)
    }
}

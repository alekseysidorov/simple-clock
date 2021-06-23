use std::time::{Duration, Instant};

use crate::{ElapsedTimer, SimpleClock};

struct StdClock {
    start_at: Instant,
}

impl StdClock {
    fn new() -> Self {
        Self {
            start_at: Instant::now(),
        }
    }
}

impl SimpleClock for StdClock {
    fn now_us(&self) -> u64 {
        let duration = Instant::now().duration_since(self.start_at);
        duration.as_micros() as u64
    }
}

fn us_to_ms_rounded(us: u64) -> u64 {
    (us as f64 / 1_000_f64).round() as u64
}

#[test]
fn test_std_clock() {
    let clock = StdClock::new();

    let time = clock.now_us();
    std::thread::sleep(Duration::from_millis(100));
    let elapsed = clock.now_us().saturating_sub(time);

    assert_eq!(us_to_ms_rounded(elapsed), 100);
}

#[test]
fn test_elapsed_timer() {
    let clock = StdClock::new();

    let timer = ElapsedTimer::new(&clock);
    std::thread::sleep(Duration::from_millis(203));
    let elapsed = timer.elapsed();

    assert_eq!(us_to_ms_rounded(elapsed), 203);
}

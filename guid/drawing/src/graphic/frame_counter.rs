use std::fmt::{Debug, Display};

pub struct FrameCounter {
    count: u32,
    tick: std::time::SystemTime,
    result: (u32, u128, f32),
}

impl FrameCounter {
    pub fn new() -> Self {
        Self {
            count: 0u32,
            tick: std::time::SystemTime::now(),
            result: (0u32, 0u128, 0f32),
        }
    }

    pub fn count(&mut self) {
        self.count += 1;
    }

    pub fn pop(&mut self) -> bool {
        match self.tick.elapsed() {
            Ok(elapsed) if elapsed.as_secs() >= 1 => {
                self.result = (self.count, elapsed.as_millis(), (self.count as f64 / elapsed.as_millis() as f64 * 1000f64) as _);
                self.count = 0;
                self.tick = std::time::SystemTime::now();
                true
            }
            _ => false
        }
    }
}

impl Display for FrameCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (count, elapsed, fps) = self.result;
        write!(f, "{:?} frames rendered in {:?} millis -> FPS= {:.2?}", count, elapsed, fps)
    }
}

impl Debug for FrameCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (count, elapsed, fps) = self.result;
        write!(f, "{:?} frames rendered in {:?} millis -> FPS= {:.2?}", count, elapsed, fps)
    }
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Decision {
    CONTINUE = 0,
    ATOMIC_HALT = 1,
}

#[repr(C)]
pub struct SafetyKernel {
    baseline: f32,
    threshold: f32,
    halted: bool,
}

impl SafetyKernel {
    pub const fn new(baseline: f32, threshold: f32) -> Self {
        Self {
            baseline,
            threshold,
            halted: false,
        }
    }

    #[inline(always)]
    fn deviation(&self, metric: f32) -> f32 {
        if metric >= self.baseline {
            metric - self.baseline
        } else {
            self.baseline - metric
        }
    }

    pub fn step(&mut self, metric: f32) -> Decision {
        if self.halted {
            return Decision::ATOMIC_HALT;
        }

        if self.deviation(metric) > self.threshold {
            self.halted = true;
            Decision::ATOMIC_HALT
        } else {
            Decision::CONTINUE
        }
    }
}

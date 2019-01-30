use crate::atomic_float::AtomicFloat;

pub struct Parameters {
    volume: AtomicFloat,
}

impl Parameters {
    pub fn new() -> Self {
        Self {
            volume: AtomicFloat::new(1.0),
        }
    }
}
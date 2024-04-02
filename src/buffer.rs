//! Sample buffering, storage, and comparison

use cortex_m::singleton;

/// Custom type for comparing signal samples
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SampleStamp(u32);

impl SampleStamp {
    pub const fn new() -> Self {
        Self(0u32)
    }
}

/// Various buffers used for managing signal samples
pub struct Buffers {
    /// Records up to 90 s of average samples to determine if a detection event occurred
    pub longterm_buffer: [u8; 45000],
    /// Counter for the most recent sample added to
    pub current_sample: SampleStamp,
    /// Indicates position time stamps for up to 10 recent detection events, comparable with `current_sample`
    pub detection_events: [SampleStamp; 10],
}

impl Buffers {
    /// Initializes static buffers for [`crate::BUFFERS`]
    pub const fn init() -> Self {
        Self {
            longterm_buffer: [0u8; 45000],
            current_sample: SampleStamp::new(),
            detection_events: [SampleStamp::new(); 10],
        }
    }
    
    /// Works with [`Buffers`] for DMA readings
    pub fn create_avg_buffer() -> &'static mut [u8; 4000] {
        singleton!(: [u8; 4000] = [0u8;4000]).unwrap()
    }
}
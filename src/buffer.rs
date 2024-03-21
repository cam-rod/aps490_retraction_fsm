use cortex_m::singleton;

/// Custom type for comparing signal samples
#[derive(Default, Ord, PartialOrd, Eq, PartialEq)]
struct SampleStamp(u32);

/// Various buffers used for managing 
pub struct Buffers {
    /// Records 2 ms of samples to calculate average voltage change of the signal
    pub avg_buffer: &'static [u8; 4000],
    /// Records up to 90 s of average samples to determine if a detection event occurred
    pub longterm_buffer: &'static [u8; 45000],
    /// Counter for the most recent sample added to 
    pub current_sample: &'static SampleStamp,
        /// Indicates position time stamps for up to 10 recent detection events, comparable with `current_sample`
    pub detection_events: &'static [SampleStamp; 10]
}

impl Default for Buffers {
    fn default() -> Self {
        Self {
            avg_buffer: singleton!(AVG_BUFFER: [u8; 4000] = [0u8; 4000]).unwrap(),
            longterm_buffer: singleton!(LONGTERM_BUFFER: [u8; 45000] = [0u8; 45000]).unwrap(),
            current_sample: singleton!(CURRENT_SAMPLE: SampleStamp = SampleStamp::default()).unwrap(),
            detection_events: singleton!(DETECTION_EVENTS: [SampleStamp; 10] = [SampleStamp::default(); 10]).unwrap(),
        }
    }
}

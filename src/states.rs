//! FSM and transitions for the detection mechanism.

pub const SYS_CLOCK_FREQ: u32 = 12_000_000;

/// States which can transition into [`Active`]
pub trait InitStates {}

/// System is initializing clocks and components
pub struct Startup;
impl InitStates for Startup {}

/// System has been manually disabled with the switch
pub struct Disabled;
impl InitStates for Disabled {}

/// System is booted and is actively checking for contact with brain material
pub struct Active;

/// A potential contact event has occurred, awaiting confirmation
pub struct PotentialContact;

/// Contact with brain detected, LED illuminated
pub struct ConfirmedContact;

/// System error, reboot required
pub struct Error;

impl Startup {
    /// Create FSM with ownership of peripherals, initialize clocks with 12 Mh
    pub fn new() -> Self {
        Self
    }
}

impl Active {
    /// Validate proper system configuration before activation
    pub fn activate(init_state: impl InitStates) {}
}

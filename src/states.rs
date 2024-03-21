//! FSM and transitions for the detection mechanism.
use crate::components::{SignalGenPwm, StatusLeds};

/// States which can transition into [`Active`]
pub trait InitState {}
/// States which indicate normal operation and/or no action required
pub trait StandbyState {}
/// States which indicate a detection event or system initialization
pub trait AlertState {}

/// System is initializing clocks and components
#[derive(Default)]
pub struct Startup {
    status_leds: Option<StatusLeds>,
    signal_gen: Option<SignalGenPwm>,
}
impl InitState for Startup {}
impl AlertState for Startup {}

/// System has been manually disabled with the switch
pub struct Disabled;
impl InitState for Disabled {}

/// System is booted and is actively checking for contact with brain material
pub struct Active;
impl StandbyState for Active {}

/// A potential contact event has occurred, awaiting confirmation
pub struct PotentialContact;
impl StandbyState for PotentialContact {}

/// Contact with brain detected, LED illuminated
pub struct ConfirmedContact;
impl AlertState for ConfirmedContact {}

/// System error, reboot required
pub struct Error;

impl Startup {
    // Stores disable switch, LEDs, and signal gen pins in FSM
    pub fn init_components(&mut self, status_leds: StatusLeds, signal_gen: SignalGenPwm) {
        // Warning status while system is initialized
        self.status_leds = Some(status_leds);
        self.signal_gen = Some(signal_gen);
    }
}

impl Active {
    /// Validate proper system configuration before activation
    pub fn activate(init_state: impl InitState) {}
}

//! FSM and transitions for the detection mechanism.

use embedded_hal::digital::PinState;
use crate::components::{DisableSwitch, StatusLeds};
use rp2040_hal::gpio::Pins;
use rp2040_hal::pac::{IO_BANK0, PADS_BANK0, RESETS};
use rp2040_hal::sio::SioGpioBank0;

pub const SYS_CLOCK_FREQ: u32 = 12_000_000;

/// States which can transition into [`Active`]
pub trait InitState {}
/// States which indicate normal operation and/or no action required
pub trait StandbyState {}
/// States which indicate a detection event or system initialization
pub trait AlertState {}

/// System is initializing clocks and components
pub struct Startup {
    disable_switch: Option<DisableSwitch>,
    status_leds: Option<StatusLeds>,
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
    /// Create FSM with ownership of peripherals, initialize clocks with 12 Mh
    pub fn new() -> Self {
        Self {
            disable_switch: None,
            status_leds: None,
        }
    }

    // Stores components in FSM
    pub fn init_components(&mut self, disable_switch: DisableSwitch, status_leds: StatusLeds) {
        self.disable_switch = Some(disable_switch);
        // Warning status while system is initialized
        self.status_leds = Some(status_leds);
    }
}

impl Active {
    /// Validate proper system configuration before activation
    pub fn activate(init_state: impl InitState) {}
}

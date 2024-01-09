use std::thread::sleep;
use std::time::{Duration, Instant};

use states::{
    Active, ContactDetected, Disabled, ErrorState, FatalError, Fsm, Resetting, Retracted,
    Retracting, Standby,
};

use crate::sensors::DisableSwitchState::SystemDisabled;
use crate::sensors::{
    startup_check, Actuator, ActuatorDirection, CapPlates, DisableSwitch, Leds, ResetButton, Saw,
};

pub mod states;

/// TODO remove when switching to interrupts
#[derive(Debug)]
pub enum FsmHandle {
    Disabled(Fsm<Disabled>),
    Standby(Fsm<Standby>),
    Active(Fsm<Active>),
    ContactDetected(Fsm<ContactDetected>),
    Retracting(Fsm<Retracting>),
    Retracted(Fsm<Retracted>),
    Resetting(Fsm<Resetting>),
    ErrorState(Fsm<ErrorState>),
    FatalError(Fsm<FatalError>),
}

impl Default for FsmHandle {
    /// Initialize the system as [`Disabled`], unless disable switch is not toggled.
    fn default() -> Self {
        Self::Disabled(Fsm::default())
    }
}

impl FsmHandle {
    pub fn enable_system(
        self,
        d: &DisableSwitch,
        r: &ResetButton,
        a: &Actuator,
        l: &mut Leds,
        p: &CapPlates,
    ) -> Self {
        match self {
            Self::Disabled(mut fsm) => {
                if startup_check(d, r, a, l, p) {
                    *l = Leds::Green;
                    Self::Standby(fsm.into())
                } else {
                    fsm.errors += 1;
                    *l = Leds::Red;
                    Self::ErrorState(fsm.into())
                }
            }
            handle => handle,
        }
    }

    /// Starts the system if the saw is running
    pub fn activate_system(self, saw: &Saw) -> Self {
        match self {
            Self::Standby(fsm) => {
                if saw.running {
                    Self::Active(fsm.into())
                } else {
                    Self::Standby(fsm)
                }
            }
            _ => self,
        }
    }

    /// Measures capacitance the first time, to validate detection. For now, assume capacitance below 0.3.
    pub fn first_contact_detection(self, plates: &CapPlates) -> Self {
        match self {
            Self::Active(fsm) => {
                if plates.discharged() {
                    Self::ContactDetected(fsm.into())
                } else {
                    Self::Active(fsm)
                }
            }
            _ => self,
        }
    }

    pub fn second_contact_detection(self, plates: &CapPlates, actuator: &mut Actuator, leds: &mut Leds) -> Self {
        sleep(Duration::from_millis(300));
        match self {
            Self::ContactDetected(mut fsm) => {
                if plates.discharged() {
                    if actuator.actuate(ActuatorDirection::Extend) == ActuatorDirection::Extend {
                        *leds = Leds::Yellow;
                        Self::Retracting(fsm.into())
                    } else {
                        fsm.errors += 1;
                        *leds = Leds::Red;
                        Self::ErrorState(fsm.into())
                    }
                } else {
                    Self::ContactDetected(fsm)
                }
            }
            _ => self,
        }
    }

    /// This function provides an example of a watchdog, using [`Instant`].
    ///
    /// A real implementation would use something like [tokio_watchdog](https://docs.rs/crate/tokio-watchdog/latest).
    ///
    /// ```
    /// use std::thread::sleep;
    /// use std::time::{Duration, Instant};
    /// let watchdog = Instant::now();
    ///
    /// while watchdog.elapsed() < Duration::from_secs(5) {
    ///     println!("Looping");
    ///     sleep(Duration::from_millis(250));
    /// }
    /// println!("woof!")
    /// ```
    pub fn retraction_complete(self, actuator: &Actuator, leds: &mut Leds) -> Self {
        let watchdog = Instant::now();
        while watchdog.elapsed() < Duration::from_secs(5) {
            match &self {
                Self::Retracting(fsm) => {
                    if actuator.extended() {
                        return Self::Retracted(fsm.into());
                    } else {
                        sleep(Duration::from_millis(200));
                    }
                }
                _ => return self,
            }
        }

        eprintln!("Watchdog timeout while waiting for saw retraction");
        *leds = Leds::Red;
        if let Self::Retracting(fsm) = self {
            Self::ErrorState(fsm.into())
        } else {
            panic!("Watchdog timeout outside of expected Retracting state!")
        }
    }

    pub fn reset_position(self, reset_button: &ResetButton, actuator: &mut Actuator, leds: &mut Leds) -> Self {
        match self {
            Self::Retracted(mut fsm) => {
                if reset_button.pressed {
                    if actuator.actuate(ActuatorDirection::Retract) == ActuatorDirection::Retract {
                        Self::Resetting(fsm.into())
                    } else {
                        fsm.errors += 1;
                        *leds = Leds::Red;
                        Self::ErrorState(fsm.into())
                    }
                } else {
                    Self::Retracted(fsm)
                }
            }
            _ => self,
        }
    }

    /// Functionally identical to [`FsmHandle::retraction_complete()`].
    pub fn reset_complete(self, actuator: &Actuator, leds: &mut Leds) -> Self {
        let watchdog = Instant::now();
        while watchdog.elapsed() < Duration::from_secs(5) {
            match &self {
                Self::Resetting(fsm) => {
                    if actuator.retracted() {
                        *leds = Leds::Green;
                        return Self::Active(fsm.into());
                    } else {
                        sleep(Duration::from_millis(200));
                    }
                }
                _ => return self,
            }
        }

        eprintln!("Watchdog timeout while waiting for saw reset");
        *leds = Leds::Red;
        if let Self::Retracting(fsm) = self {
            Self::ErrorState(fsm.into())
        } else {
            panic!("Watchdog timeout outside of expected Reset state!")
        }
    }

    /// This will fully lockout the system if 3 or more errors.
    ///
    /// The system cannot be re-enabled without a full power cycle. _Toggling [`DisableSwitch`] will not work._
    pub fn error_lockout(self) -> Self {
        match self {
            Self::ErrorState(fsm) => {
                if fsm.errors >= 3 {
                    Self::FatalError(fsm.into())
                } else {
                    Self::ErrorState(fsm)
                }
            }
            _ => self,
        }
    }

    /// The system can be safely disabled from [`Standby`] or either of the error states ([`ErrorState`] or [`FatalError`]).
    pub fn disable_system(self, switch: &DisableSwitch, leds: &mut Leds) -> Self {
        if switch.system == SystemDisabled {
            match self {
                Self::Standby(fsm) => {
                    *leds = Leds::Off;
                    Self::Disabled(fsm.into())
                },
                Self::ErrorState(fsm) => {
                    *leds = Leds::Off;
                    Self::Disabled(fsm.into())
                },
                Self::FatalError(fsm) => {
                    *leds = Leds::Off;
                    Self::Disabled(fsm.into())
                },
                _ => self,
            }
        } else {
            self
        }
    }
}

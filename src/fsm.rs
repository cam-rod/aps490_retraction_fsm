use crate::fsm::states::{
    Active, ContactDetected, Disabled, ErrorState, FatalError, Fsm, Resetting, Retracted,
    Retracting, Standby,
};
use crate::sensors::ActuatorDirection::{Extend, Retract};
use crate::sensors::{startup_check, Actuator, CapPlates, DisableSwitch, Leds, ResetButton, Saw};
use std::thread::sleep;
use std::time::{Duration, Instant};

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
    Error(Fsm<ErrorState>),
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
        l: &Leds,
        p: &CapPlates,
    ) -> Self {
        match self {
            Self::Disabled(fsm) => {
                if startup_check(d, r, a, l, p) {
                    Self::Standby(fsm.into())
                } else {
                    Self::Disabled(fsm)
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

    pub fn second_contact_detection(self, plates: &CapPlates, actuator: &mut Actuator) -> Self {
        sleep(Duration::from_millis(300));
        match self {
            Self::ContactDetected(mut fsm) => {
                if plates.discharged() {
                    if actuator.actuate(Extend) == Extend {
                        Self::Retracting(fsm.into())
                    } else {
                        fsm.errors += 1;
                        Self::Error(fsm.into())
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
    pub fn retraction_complete(self, actuator: &Actuator) -> Self {
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
        if let Self::Retracting(fsm) = self {
            Self::Error(fsm.into())
        } else {
            panic!("Watchdog timeout outside of expected Retracting state!")
        }
    }

    pub fn reset_position(self, reset_button: &ResetButton, actuator: &mut Actuator) -> Self {
        match self {
            Self::Retracted(mut fsm) => {
                if reset_button.pressed {
                    if actuator.actuate(Retract) == Retract {
                        Self::Resetting(fsm.into())
                    } else {
                        fsm.errors += 1;
                        Self::Error(fsm.into())
                    }
                } else {
                    Self::Retracted(fsm)
                }
            }
            _ => self,
        }
    }

    /// Functionally identical to [`FsmHandle::retraction_complete()`].
    pub fn reset_complete(self, actuator: &Actuator) -> Self {
        let watchdog = Instant::now();
        while watchdog.elapsed() < Duration::from_secs(5) {
            match &self {
                Self::Resetting(fsm) => {
                    if actuator.retracted() {
                        return Self::Active(fsm.into());
                    } else {
                        sleep(Duration::from_millis(200));
                    }
                }
                _ => return self,
            }
        }

        eprintln!("Watchdog timeout while waiting for saw reset");
        if let Self::Retracting(fsm) = self {
            Self::Error(fsm.into())
        } else {
            panic!("Watchdog timeout outside of expected Reset state!")
        }
    }
}

pub mod states {
    use retract_fsm_transition::transition_from;

    #[derive(Debug)]
    pub struct Fsm<S> {
        pub errors: isize,
        #[allow(dead_code)]
        state: S,
    }

    impl Default for Fsm<Disabled> {
        fn default() -> Self {
            Self {
                errors: 0,
                state: Disabled,
            }
        }
    }

    /// System is disabled, and will not engage during operation
    #[derive(Debug)]
    pub struct Disabled;

    /// System is enabled, saw is not currently running
    #[derive(Debug)]
    #[transition_from(Disabled)]
    pub struct Standby;

    /// System is enabled and looking for changes in conductance/current, saw is running
    #[derive(Debug)]
    #[transition_from(Standby, Resetting)]
    pub struct Active;

    /// Intermediate state, waiting minimum delay period for further confirmation
    #[derive(Debug)]
    #[transition_from(Active)]
    pub struct ContactDetected;

    /// Contact has been confirmed, begin retraction
    #[derive(Debug)]
    #[transition_from(ContactDetected)]
    pub struct Retracting;

    /// Saw blade has been retracted, position needs to be reset
    #[derive(Debug)]
    #[transition_from(Retracting)]
    pub struct Retracted;

    /// Saw blade is returning to running position
    #[derive(Debug)]
    #[transition_from(Retracted)]
    pub struct Resetting;

    /// An error was detected in a previous state. Reset is required to restore operation
    #[derive(Debug)]
    #[transition_from(Active, ContactDetected, Retracting, Retracted, Resetting)]
    pub struct ErrorState;

    /// Three errors in a row have occurred. System will not reset without a full power cycle
    #[derive(Debug)]
    #[transition_from(ErrorState)]
    pub struct FatalError;

    // Also want to include an error counter to determine number of errors, and force disable after 3 errors until full system reboot
}

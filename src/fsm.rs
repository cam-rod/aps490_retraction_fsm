use crate::fsm::states::{
    Active, ContactDetected, Disabled, ErrorState, FatalError, Fsm, Resetting, Retracted,
    Retracting, Standby,
};
use crate::sensors::{startup_check, Actuator, CapPlates, DisableSwitch, Leds, ResetButton, Saw};

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
}

pub mod states {
    use retract_fsm_transition::transition_from;

    #[derive(Debug)]
    pub struct Fsm<S> {
        errors: isize,
        state: S,
    }

    impl Default for Fsm<Disabled> {
        fn default() -> Self {
            Self::default()
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
    pub struct ContactDetected;

    /// Contact has been confirmed, begin retraction
    #[derive(Debug)]
    pub struct Retracting;

    /// Saw blade has been retracted, position needs to be reset
    #[derive(Debug)]
    pub struct Retracted;

    /// Saw blade is returning to running position
    #[derive(Debug)]
    pub struct Resetting;

    /// An error was detected in a previous state. Reset is required to restore operation
    #[derive(Debug)]
    pub struct ErrorState;

    /// Three errors in a row have occurred. System will not reset without a full power cycle
    #[derive(Debug)]
    pub struct FatalError;

    // Also want to include an error counter to determine number of errors, and force disable after 3 errors until full system reboot
}

use crate::sensors::startup_check;

/// TODO remove when switching to interrupts
#[derive(Debug, Clone)]
pub enum FsmHandle {
    Disabled(Fsm<Disabled>),
    Standby(Fsm<Standby>),
    Active(Fsm<Active>),
    ContactDetected(Fsm<ContactDetected>),
    Retracting(Fsm<Retracting>),
    Retracted(Fsm<Retracted>),
    Resetting(Fsm<Resetting>),
    Error(Fsm<Error>),
    FatalError(Fsm<FatalError>),
}

impl Default for FsmHandle {
    /// Initialize the system as [`Disabled`], unless disable switch is not toggled.
    fn default() -> Self {
        Self::Disabled(Fsm {
            errors: 0,
            state: Disabled,
        })
    }
}

impl FsmHandle {
    pub fn enable_system(self) -> Self {
        match self {
            Self::Disabled(fsm) => {
                if startup_check() {
                    Self::Standby(Fsm {
                        errors: fsm.errors,
                        state: Standby,
                    })
                } else {
                    Self::Disabled(fsm)
                }
            }
            handle => handle,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Fsm<S> {
    errors: isize,
    state: S,
}

/// System is disabled, and will not engage during operation
#[derive(Debug, Copy, Clone)]
pub struct Disabled;

/// System is enabled, saw is not currently running
#[derive(Debug, Copy, Clone)]
pub struct Standby;

/// System is enabled and looking for changes in conductance/current, saw is running
#[derive(Debug, Copy, Clone)]
pub struct Active;

/// Intermediate state, waiting minimum delay period for further confirmation
#[derive(Debug, Copy, Clone)]
pub struct ContactDetected;

/// Contact has been confirmed, begin retraction
#[derive(Debug, Copy, Clone)]
pub struct Retracting;

/// Saw blade has been retracted, position needs to be reset
#[derive(Debug, Copy, Clone)]
pub struct Retracted;

/// Saw blade is returning to running position
#[derive(Debug, Copy, Clone)]
pub struct Resetting;

/// An error was detected in a previous state. Reset is required to restore operation
#[derive(Debug, Copy, Clone)]
pub struct Error;

/// Three errors in a row have occurred. System will not reset without a full power cycle
#[derive(Debug, Copy, Clone)]
pub struct FatalError;

// Also want to include an error counter to determine number of errors, and force disable after 3 errors until full system reboot

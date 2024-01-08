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
#[transition_from(Standby, ErrorState, FatalError)]
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
///
/// A full implementation should also check for errors in [`Standby`].
#[derive(Debug)]
#[transition_from(Disabled, Active, ContactDetected, Retracting, Retracted, Resetting)]
pub struct ErrorState;

/// Three errors in a row have occurred. System will not reset without a full power cycle
#[derive(Debug)]
#[transition_from(ErrorState)]
pub struct FatalError;

// Also want to include an error counter to determine number of errors, and force disable after 3 errors until full system reboot

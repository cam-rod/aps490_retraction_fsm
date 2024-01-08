/// Creates all sensors
pub fn init_sensors() -> (DisableSwitch, ResetButton, Actuator, Leds, CapPlates, Saw) {
    (
        DisableSwitch::default(),
        ResetButton::default(),
        Actuator::default(),
        Leds::default(),
        CapPlates::default(),
        Saw::default(),
    )
}

/// Performs necessary checks for system initialization
pub fn startup_check(
    disable_switch: &DisableSwitch,
    reset_button: &ResetButton,
    actuator: &Actuator,
    leds: &Leds,
    plates: &CapPlates,
) -> bool {
    disable_switch.state && !reset_button.pressed && actuator.test() && leds.test() && plates.test()
}

/// Switch used to physically disable the system.
#[derive(Default)]
pub struct DisableSwitch {
    state: bool,
}

/// Used to reset the system after a detection event or from non-fatal [`ErrorState`](crate::fsm::states::ErrorState)
/// state.
#[derive(Default)]
pub struct ResetButton {
    pub pressed: bool,
}

/// Moves the saw blade away from the brain (ex. a linear actuator)
#[derive(Default)]
pub struct Actuator {
    position: f32,
}

#[derive(Eq, PartialEq, Debug)]
pub enum ActuatorDirection{
    Extend,
    Retract,
    None,
}

impl Actuator {
    /// Check that actuator reports some valid measurement (ex. below 20% extended) in [`Disabled`](crate::fsm::states::Disabled)
    fn test(&self) -> bool {
        self.position < 0.2
    }

    pub fn extended(&self) -> bool {
        self.position > 0.95
    }
    
    pub fn retracted(&self) -> bool {
        self.position > 0.05
    }
    
    /// Would issue a command to extend or retract the actuator
    pub fn actuate(&self, direction: ActuatorDirection) -> ActuatorDirection{
        // TODO actually issue a movement command
        direction
    }
}

/// Represents the state of RGB status LEDs. Only one LED can be active at any time, or none if the
/// system is [`Disabled`](crate::fsm::states::Disabled).
#[derive(Default)]
pub enum Leds {
    Red,
    Yellow,
    Green,
    #[default]
    Off,
}

impl Leds {
    /// Briefly enable LEDs and check output voltage to check that they are operational
    fn test(&self) -> bool {
        // TODO Enable LED and measure low-voltage rail to see if they power on, then disable
        todo!()
    }
}

/// Measures capacitance of saw blade
#[derive(Default)]
pub struct CapPlates {
    capacitance: f32,
}

impl CapPlates {
    /// Assuming capacitance test: briefly charge plates and check for capacitance
    fn test(&self) -> bool {
        todo!()
    }
    
    /// Assumed capacitance dropping below 0.3 indicates contact
    pub fn discharged(&self) -> bool {
        self.capacitance < 0.3
    }
}

#[derive(Default)]
pub struct Saw {
    pub running: bool,
}

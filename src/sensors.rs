// TODO consider enums for each sensor type

/// Performs necessary checks for system initialization
pub fn startup_check() -> bool {
    is_disable_switch() && !is_reset_button() && actuator_test() && led_test() && plates_test() 
}

fn is_disable_switch() -> bool {
    todo!()
}

/// Returns `true` if pressed
fn is_reset_button() -> bool {
    todo!()
}

/// Check that actuator reports some valid measurement (ex. below 20% extended) in [`Disabled`](crate::fsm::Disabled)
fn actuator_test() -> bool {
    todo!()
}

/// Briefly enable LEDs and check output voltage to check that they are operational
fn led_test() -> bool {
    // TODO Enable LED and measure low-voltage rail to see if they power on, then disable
    todo!()
}

/// Assuming capacitance test: briefly charge plates and check for capacitance
fn plates_test() -> bool {
    todo!()
}
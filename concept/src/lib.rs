use fsm::FsmHandle;
use sensors::init_sensors;

pub mod fsm;
pub mod sensors;

// TODO replace with interrupts

/// The main execution loop, non-interrupt edition.
///
/// This function is missing a watchdog. For an example, see [`FsmHandle::retraction_complete()`].
pub fn main_loop() {
    let mut fsm = FsmHandle::default();
    #[allow(unused_mut)]
    let (mut disable_switch, mut reset_button, mut actuator, mut leds, mut plates, saw) =
        init_sensors();

    // TODO add regular, proper error checking
    loop {
        fsm = fsm.enable_system(&disable_switch, &reset_button, &actuator, &mut leds, &plates);
        fsm = fsm.activate_system(&saw);

        // Capacitance test
        fsm = fsm.first_contact_detection(&plates);
        fsm = fsm.second_contact_detection(&plates, &mut actuator, &mut leds);

        // Retraction and reset
        fsm = fsm.retraction_complete(&actuator, &mut leds);
        fsm = fsm.reset_position(&reset_button, &mut actuator, &mut leds);
        fsm = fsm.reset_complete(&actuator, &mut leds);

        // Error handling
        fsm = fsm.error_lockout();
        fsm = fsm.disable_system(&disable_switch, &mut leds);
    }
}

use crate::fsm::FsmHandle;
use crate::sensors::init_sensors;

mod fsm;
mod sensors;

// TODO replace with interrupts
pub fn main_loop() {
    let mut fsm = FsmHandle::default();
    let (mut disable_switch, mut reset_button, mut actuator, mut leds, mut plates, saw) = init_sensors();

    loop {
        fsm = fsm.enable_system(&disable_switch, &reset_button, &actuator, &leds, &plates);
        fsm = fsm.activate_system(&saw)
    }
}

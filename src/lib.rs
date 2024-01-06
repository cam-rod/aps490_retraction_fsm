use crate::fsm::FsmHandle;

mod fsm;
mod sensors;

// TODO replace with interrupts
pub fn main_loop() {
    let mut fsm = FsmHandle::default();
    
    loop {
        fsm = fsm.enable_system()
    }
} 
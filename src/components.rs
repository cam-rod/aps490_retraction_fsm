//! I/O components, including all interrupts
use embedded_hal::{digital::PinState, pwm::SetDutyCycle};
use rp2040_hal::{
    gpio::{
        bank0::{Gpio22, Gpio6, Gpio7, Gpio8, Gpio9},
        FunctionNull, FunctionSio, Interrupt, Pin, PullDown, SioInput, SioOutput,
    },
    pac::interrupt,
    pwm::{Channel, FreeRunning, Pwm3, Slice, A},
};
use rp2040_hal::gpio::Interrupt::EdgeHigh;

use crate::DISABLE_SWTICH;

/// Switch which will interrupt operations and put system in [`Disabled`](crate::states::Disabled) state.
pub struct DisableSwitch(Pin<Gpio9, FunctionSio<SioInput>, PullDown>);

impl DisableSwitch {
    /// Configures disable switch and interrupt
    pub fn configure(gpio9: Pin<Gpio9, FunctionNull, PullDown>) {
        let switch = gpio9.into_pull_down_input();
        switch.set_interrupt_enabled(Interrupt::EdgeHigh, true);
        critical_section::with(|cs| {
            DISABLE_SWTICH.borrow(cs).replace(Some(Self(switch)));
        });
    }
}

/// ISR for GPIO pins
///
/// Lazily takes ownership of [`DISABLE_SWTICH`] as it will not be used again in the main runtime again.
#[interrupt]
fn IO_IRQ_BANK0() {
    static mut DISABLE_SWITCH_IRQ: Option<DisableSwitch> = None;

    if DISABLE_SWITCH_IRQ.is_none() {
        critical_section::with(|cs| {
            *DISABLE_SWITCH_IRQ = DISABLE_SWTICH.borrow(cs).take();
        });
    }
    
    if let Some(switch) = DISABLE_SWITCH_IRQ {
        if switch.0.interrupt_status(EdgeHigh)
    }
}

/// Green, yellow, and red LED which display system status
pub struct StatusLeds {
    good_led: Pin<Gpio6, FunctionSio<SioOutput>, PullDown>,
    alert_led: Pin<Gpio7, FunctionSio<SioOutput>, PullDown>,
    error_led: Pin<Gpio8, FunctionSio<SioOutput>, PullDown>,
}

impl StatusLeds {
    pub fn init(
        good_led: Pin<Gpio6, FunctionNull, PullDown>,
        alert_led: Pin<Gpio7, FunctionNull, PullDown>,
        error_led: Pin<Gpio8, FunctionNull, PullDown>,
    ) -> Self {
        Self {
            good_led: good_led.into_push_pull_output_in_state(PinState::Low),
            alert_led: alert_led.into_push_pull_output_in_state(PinState::High),
            error_led: error_led.into_push_pull_output_in_state(PinState::Low),
        }
    }
}

/// Generates a 100 kHz signal for contact detection.
pub struct SignalGenPwm {
    pwm3a: Channel<Slice<Pwm3, FreeRunning>, A>,
}

impl SignalGenPwm {
    pub fn init(
        mut pwm3a: Channel<Slice<Pwm3, FreeRunning>, A>,
        gpio22: Pin<Gpio22, FunctionNull, PullDown>,
    ) -> Self {
        pwm3a.output_to(gpio22);
        pwm3a.set_duty_cycle_percent(50u8).unwrap();
        Self { pwm3a }
    }
}

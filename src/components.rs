use embedded_hal::digital::PinState;
use embedded_hal::pwm::SetDutyCycle;
use rp2040_hal::gpio::{
    bank0::{Gpio22, Gpio6, Gpio7, Gpio8, Gpio9},
    FunctionNull, FunctionSio, Pin, PullDown, SioInput, SioOutput,
};
use rp2040_hal::pwm::{Channel, FreeRunning, Pwm3, Slice, A};

/// Switch which will interrupt operations and put system in [`Disabled`](crate::states::Disabled) state.
pub type DisableSwitch = Pin<Gpio9, FunctionSio<SioInput>, PullDown>;

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

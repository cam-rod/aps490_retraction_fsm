use embedded_hal::digital::PinState;
use rp2040_hal::gpio::{bank0::{Gpio6, Gpio7, Gpio8, Gpio9}, FunctionNull, FunctionSio, Pin, PullDown, SioInput, SioOutput};

/// Green, yellow, and red LED which display system status
pub struct StatusLeds {
    good_led: Pin<Gpio6, FunctionSio<SioOutput>, PullDown>,
    alert_led: Pin<Gpio7, FunctionSio<SioOutput>, PullDown>,
    error_led: Pin<Gpio8, FunctionSio<SioOutput>, PullDown>,
}

impl StatusLeds {
    pub fn init(
        good_led:  Pin<Gpio6, FunctionNull, PullDown>,
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

pub(crate) type DisableSwitch = Pin<Gpio9, FunctionSio<SioInput>, PullDown>;
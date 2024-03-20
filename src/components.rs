use embedded_hal::digital::PinState;
use rp2040_hal::gpio::{
    bank0::{Gpio6, Gpio7, Gpio8, Gpio9},
    FunctionSio, Pin, Pins, PullDown, SioInput, SioOutput,
};
use rp2040_hal::pac::{IO_BANK0, PADS_BANK0, RESETS};
use rp2040_hal::sio::SioGpioBank0;
use crate::states::Startup;

/// Green, yellow, and red LED which display system status
pub struct StatusLeds {
    good_led: Pin<Gpio6, FunctionSio<SioOutput>, PullDown>,
    alert_led: Pin<Gpio7, FunctionSio<SioOutput>, PullDown>,
    error_led: Pin<Gpio8, FunctionSio<SioOutput>, PullDown>,
}

impl StatusLeds {
    pub fn init(
        good_led: Pin<Gpio6, FunctionSio<SioOutput>, PullDown>,
        alert_led: Pin<Gpio7, FunctionSio<SioOutput>, PullDown>,
        error_led: Pin<Gpio8, FunctionSio<SioOutput>, PullDown>,
    ) -> Self {
        Self {
            good_led,
            alert_led,
            error_led,
        }
    }
}

pub(crate) type DisableSwitch = Pin<Gpio9, FunctionSio<SioInput>, PullDown>;
//! I/O components, including all interrupts
use cortex_m_rt::exception;
use defmt::warn;
use embedded_hal::{
    digital::{InputPin, PinState},
    pwm::SetDutyCycle,
};
use rp2040_hal::{
    gpio::{
        bank0::{Gpio22, Gpio6, Gpio7, Gpio8, Gpio9},
        FunctionNull, FunctionSio, Pin, PullDown, SioInput, SioOutput,
    },
    pwm::{Channel, FreeRunning, Pwm3, Slice, A},
};

use crate::DISABLE_SWITCH;

/// Switch which will interrupt operations and put system in [`Disabled`](crate::states::Disabled) state.
pub struct DisableSwitch(Pin<Gpio9, FunctionSio<SioInput>, PullDown>);

impl DisableSwitch {
    /// Configures disable switch, and places in [`DISABLE_SWITCH`]
    pub fn configure(gpio9: Pin<Gpio9, FunctionNull, PullDown>) {
        let switch = gpio9.into_pull_down_input();

        critical_section::with(|cs| {
            DISABLE_SWITCH.borrow(cs).replace(Some(Self(switch)));
        });
    }
}

/// LEDs which display system status
pub struct StatusLeds {
    /// Green LED, for [`Active`](crate::states::Active) and
    /// [`PotentialContact`](crate::states::PotentialContact)
    good_led: Pin<Gpio6, FunctionSio<SioOutput>, PullDown>,
    /// Yellow LED, for [`Startup`](crate::states::Startup) and
    /// [`ConfirmedContact`](crate::states::ConfirmedContact)
    alert_led: Pin<Gpio7, FunctionSio<SioOutput>, PullDown>,
    /// Red LED, for [`Error`](crate::states::Error)
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
    /// PWM channel 3A, connects to [`Gpio22`]
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

/// ISR for SysTick, used for checking [`DisableSwitch`] with debouncing
///
/// Lazily takes ownership of [`DISABLE_SWITCH`] as it will not be used again in the main runtime again.
#[exception]
fn SysTick() {
    static mut DISABLE_SWITCH_IRQ: Option<DisableSwitch> = None;
    static mut SWITCH_DEBOUNCED: bool = false;
    static mut IS_DISABLED: bool = false;

    if DISABLE_SWITCH_IRQ.is_none() {
        critical_section::with(|cs| {
            *DISABLE_SWITCH_IRQ = DISABLE_SWITCH.borrow(cs).take();
        });
    }

    if let Some(switch) = DISABLE_SWITCH_IRQ {
        if switch
            .0
            .is_high()
            .expect("Unable to check disable switch state")
        {
            if SWITCH_DEBOUNCED == &false {
                *SWITCH_DEBOUNCED = true;
            } else {
                *SWITCH_DEBOUNCED = false;
                match IS_DISABLED {
                    false => {
                        warn!("System disabled");
                        *IS_DISABLED = true
                    }
                    true => {
                        warn!("System enabled");
                        *IS_DISABLED = false
                    }
                }
            }
        }
    }
}

//! Provides contact detection with a high-conductivity surface (such as brain tissue)
//! for an autopsy saw.
#![no_std]
#![no_main]

mod components;
mod states;

use defmt::{info, warn};
#[allow(unused_imports)]
use defmt_rtt as _;
#[allow(unused_imports)]
use panic_probe as _;

use crate::components::{SignalGenPwm, StatusLeds};
use rp2040_hal::gpio::Pins;
use rp2040_hal::pwm::Slices;
use rp2040_hal::{Adc, clocks::init_clocks_and_plls, entry, fugit::RateExtU32, pac, prelude::*, sio::Sio, watchdog::Watchdog};
use rp2040_hal::dma::DMAExt;
use states::{Startup, SYS_CLOCK_FREQ};

/// Target 200 kHz (2x output freq) for signal generation
static TARGET_TOP_FREQ_KHZ: u16 = 200;

#[entry]
fn main() -> ! {
    info!("Detection system startup");
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
    let mut startup_fsm = Startup::new();

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let mut clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Attempt to switch system to 24 MHz for efficiency
    clocks
        .system_clock
        .configure_clock(&clocks.reference_clock, SYS_CLOCK_FREQ.Hz())
        .unwrap_or_else(|err| {
            warn!(
                "Unable to downscale clock speed: {}\nClocks will continue to run at {}",
                err,
                clocks.system_clock.freq().to_Hz()
            )
        });

    // Initialize components
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut pwm_slices = Slices::new(pac.PWM, &mut pac.RESETS);
    pwm_slices
        .pwm3
        .set_top((clocks.system_clock.freq().to_kHz() as u16 / TARGET_TOP_FREQ_KHZ) - 1);
    
    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);
    let mut adc_pin0 = rp2040_hal::adc::AdcPin::new(pins.gpio26.into_floating_input()).unwrap();
    let mut dma = pac.DMA.split(&mut pac.RESETS);
    let mut readings_fifo = adc.build_fifo()
        .set_channel(&mut adc_pin0)
        // Ex. 24 MHz samples
        .clock_divider((clocks.system_clock.freq().to_kHz() as u16 / (TARGET_TOP_FREQ_KHZ/4)) - 1, 0)
        .shift_8bit()
        .enable_dma()
        .start_paused();
    
    startup_fsm.init_components(
        pins.gpio9.into_pull_down_input(),
        StatusLeds::init(pins.gpio6, pins.gpio7, pins.gpio8),
        SignalGenPwm::init(pwm_slices.pwm3.channel_a, pins.gpio22),
    );
    todo!();

    // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
    // on-board LED, it might need to be changed.
    //
    // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead.
    // One way to do that is by using [embassy](https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/wifi_blinky.rs)
    //
    // If you have a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
    // LED to one of the GPIO pins, and reference that pin here. Don't forget adding an appropriate resistor
    // in series with the LED.
}

// End of file

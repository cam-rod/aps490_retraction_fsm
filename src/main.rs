//! Provides contact detection with a high-conductivity surface (such as brain tissue)
//! for an autopsy saw.
#![no_std]
#![no_main]

use defmt::{info, warn};
#[allow(unused_imports)]
use defmt_rtt as _;
#[allow(unused_imports)]
use panic_probe as _;
use rp2040_hal::{
    clocks::init_clocks_and_plls, dma::DMAExt, entry, fugit::RateExtU32, gpio::Pins, pac,
    prelude::*, pwm::Slices, sio::Sio, watchdog::Watchdog, Adc,
};

use crate::{
    buffer::Buffers,
    components::{SignalGenPwm, StatusLeds},
    states::{Startup, SYS_CLOCK_FREQ},
};

mod buffer;
mod components;
mod states;

/// Frequency of detection signal is 100 kHz
static SIGNAL_FREQ_KHZ: u32 = 100;
static mut BUFFERS: Buffers = Buffers::default();

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

    // Initialize signal generator
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut pwm_slices = Slices::new(pac.PWM, &mut pac.RESETS);
    pwm_slices
        .pwm3
        // Ex. 24 MHz clock generates 100 kHz signal ->  240 ticks per PWM cycle (`top`)
        // with 50% duty cycle
        .set_top(((clocks.system_clock.freq().to_kHz() / SIGNAL_FREQ_KHZ) - 1) as u16);

    
    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);
    let mut adc_pin0 = rp2040_hal::adc::AdcPin::new(pins.gpio26.into_floating_input()).unwrap();
    let mut dma = pac.DMA.split(&mut pac.RESETS);
    let mut readings_fifo = adc
        .build_fifo()
        .set_channel(&mut adc_pin0)
        // Ex. 24 MHz clock at 200 ksamples/s (2x SIGNAL_FREQ_KHZ) -> sample every 120 ticks
        .clock_divider(
            ((clocks.system_clock.freq().to_kHz() / (2 * SIGNAL_FREQ_KHZ)) - 1) as u16,
            0,
        )
        .shift_8bit()
        .enable_dma()
        .start_paused();

    startup_fsm.init_components(
        pins.gpio9.into_pull_down_input(),
        StatusLeds::init(pins.gpio6, pins.gpio7, pins.gpio8),
        SignalGenPwm::init(pwm_slices.pwm3.channel_a, pins.gpio22),
    );
    todo!();
}

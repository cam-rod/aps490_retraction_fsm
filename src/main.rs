//! Provides contact detection with a high-conductivity surface (such as brain tissue)
//! for an autopsy saw.
#![no_std]
#![no_main]

use defmt::{info, warn};
#[allow(unused_imports)]
use defmt_rtt as _;
use embedded_hal::digital::OutputPin;
#[allow(unused_imports)]
use panic_probe as _;

use aps490_retraction_fsm::init_components;
use aps490_retraction_fsm::states::{Startup, SYS_CLOCK_FREQ};
use rp2040_hal::{
    clocks::{init_clocks_and_plls, Clock},
    entry,
    gpio::Pins,
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    info!("Detection system startup");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
    let startup_fsm = Startup::new();

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
        .configure_clock(&clocks.reference_clock, SYS_CLOCK_FREQ.into())
        .unwrap_or_else(|err| {
            warn!(
                "Unable to downscale clock speed: {}\nClocks will continue to run at {}",
                err,
                clocks.system_clock.freq()
            )
        });

    init_components(&startup_fsm);
    todo!();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
    // on-board LED, it might need to be changed.
    //
    // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead.
    // One way to do that is by using [embassy](https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/wifi_blinky.rs)
    //
    // If you have a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
    // LED to one of the GPIO pins, and reference that pin here. Don't forget adding an appropriate resistor
    // in series with the LED.
    let mut led_pin = pins.gpio25.into_push_pull_output();

    loop {
        info!("on!");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        info!("off!");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}

// End of file

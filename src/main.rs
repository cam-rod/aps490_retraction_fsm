//! Provides contact detection with a high-conductivity surface (such as brain tissue)
//! for an autopsy saw.
#![no_std]
#![no_main]

extern crate alloc;

use core::cell::RefCell;

use cortex_m::peripheral::syst::SystClkSource;
use critical_section::Mutex;
use defmt::{info, warn};
#[allow(unused_imports)]
use defmt_rtt as _;
#[allow(unused_imports)]
use panic_probe as _;
use rp2040_hal::adc::{AdcFifo, DmaReadTarget};
use rp2040_hal::dma::SingleChannel;
use rp2040_hal::{
    clocks::init_clocks_and_plls, dma, dma::DMAExt, entry, fugit::RateExtU32, gpio::Pins, pac,
    prelude::*, pwm::Slices, sio::Sio, watchdog::Watchdog, Adc,
};

use crate::{
    buffer::Buffers,
    components::{DisableSwitch, SignalGenPwm, StatusLeds},
    states::Startup,
};

pub mod buffer;
pub mod components;
pub mod states;

/// Second-stage bootloader, from [rp2040-boot2](https://docs.rs/rp2040-boot2)
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
/// External high-speed crystal on the pico board is 12Mhz
pub const XOSC_FREQ_HZ: u32 = 12_000_000;
/// Attempt to run system clock at 24 MHz
pub const SYS_CLOCK_FREQ: u32 = 24_000_000;
/// Frequency of detection signal is 100 kHz
pub static SIGNAL_GEN_FREQ_HZ: u32 = 100_000;

/// See [`DisableSwitch`]
pub static DISABLE_SWITCH: Mutex<RefCell<Option<DisableSwitch>>> = Mutex::new(RefCell::new(None));


type Readings = (
    AdcFifo<'static, u8>,
    &'static mut [u8; 4000],
    Option<dma::single_buffer::Transfer<
        dma::Channel<dma::CH0>,
        DmaReadTarget<u8>,
        &'static mut [u8; 4000],
    >>,
)
/// Transfer ownership of readings buffer to IRQ. See [`AvgBuffer`]
pub static ADC_READINGS: Mutex<
    RefCell<
        Option<Readings>,
    >,
> = Mutex::new(RefCell::new(None));

/// See [`Buffers`]
pub static mut BUFFERS: Buffers = Buffers::init();
/// See [`AvgBuffer`]
pub static mut AVG_BUFFER: Option<&'static mut [u8; 4000]> = None;

/// Initializes main system, in [`states::Startup`]

#[entry]
fn main() -> ! {
    info!("Detection system startup");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
    let mut startup_fsm = Startup::default();

    let mut clocks = init_clocks_and_plls(
        XOSC_FREQ_HZ,
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
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Initialize signal generator
    let mut pwm_slices = Slices::new(pac.PWM, &mut pac.RESETS);
    pwm_slices
        .pwm3
        // Ex. 24 MHz clock generates 100 kHz signal ->  240 clk cycles per PWM cycle (`top`)
        // with 50% duty cycle
        .set_top(((clocks.system_clock.freq().to_Hz() / SIGNAL_GEN_FREQ_HZ) - 1) as u16);

    // Initialize ADC
    let avg_buffer = Buffers::create_avg_buffer();
    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);
    let mut adc_pin0 = rp2040_hal::adc::AdcPin::new(pins.gpio26.into_floating_input()).unwrap();
    let mut readings_fifo = adc
        .build_fifo()
        .set_channel(&mut adc_pin0)
        // Ex. 24 MHz clock at 200 ksamples/s (2x SIGNAL_FREQ_KHZ) -> sample every 120 clk cycles
        .clock_divider(
            ((clocks.system_clock.freq().to_Hz() / (2 * SIGNAL_GEN_FREQ_HZ)) - 1) as u16,
            0,
        )
        .shift_8bit()
        .enable_dma()
        .start_paused();
    let mut dma = pac.DMA.split(&mut pac.RESETS);
    let init_readings_dma =
        dma::single_buffer::Config::new(dma.ch0, readings_fifo.dma_read_target(), avg_buffer)
            .start();
    critical_section::with(|cs| {
        // Load variables into mutex, and then start the ADC readings
        ADC_READINGS
            .borrow(cs)
            .replace(Some((readings_fifo, avg_buffer, Some(init_readings_dma))))
            .as_mut()
            .unwrap()
            .0
            .resume();
    });
    dma.ch0.enable_irq0();

    startup_fsm.init_components(
        StatusLeds::init(pins.gpio6, pins.gpio7, pins.gpio8),
        SignalGenPwm::init(pwm_slices.pwm3.channel_a, pins.gpio22),
    );

    // Initialize disable switch and configure SysTick timer (done last for this reason)
    DisableSwitch::configure(pins.gpio9);
    let mut sys_tick = core.SYST;
    sys_tick.set_clock_source(SystClkSource::Core); // 1 us per tick
    sys_tick.set_reload(20_000); // 20 ms
    sys_tick.clear_current();
    sys_tick.enable_interrupt();
    sys_tick.enable_counter();
    
    loop {}
}

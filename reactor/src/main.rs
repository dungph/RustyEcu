#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]

use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use rtic::app;
use {defmt_rtt as _, panic_probe as _};

pub mod pac {
    pub use embassy_stm32::pac::Interrupt as interrupt;
    pub use embassy_stm32::pac::*;
}

#[app(device = pac, peripherals = false, dispatchers = [SPI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_cx: init::Context) -> (Shared, Local) {
        // Initialize the systick interrupt & obtain the token to prove that we did

        let p = embassy_stm32::init(Default::default());
        defmt::info!("Hello World!");

        let mut led = Output::new(p.PC6, Level::High, Speed::Low);
        defmt::info!("high");
        led.set_high();

        // Schedule the blinking task
        blink::spawn(led).ok();

        (Shared {}, Local {})
    }

    #[task()]
    async fn blink(_cx: blink::Context, mut led: Output<'static, embassy_stm32::peripherals::PC6>) {
        let mut state = true;
        loop {
            defmt::info!("blink");
            if state {
                led.set_high();
            } else {
                led.set_low();
            }
            state = !state;
            Timer::after_secs(1).await;
        }
    }
}

#![no_std]
#![no_main]

    //clock::CpuClock,
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    main,
};
use log::info;

#[main]
fn main() -> ! {
    // generator version: 0.2.2

    // let _config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Set GPIO21 as output and set state high initially
    // This is for FireBeele 2
    let mut led = Output::new(peripherals.GPIO21, Level::Low);

    led.set_high();

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    loop {
        led.toggle();
        info!("Toggle LED");
        delay.delay_millis(500);
    }
}

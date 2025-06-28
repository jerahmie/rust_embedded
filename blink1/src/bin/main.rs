#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::IO,
    main,
};
use log::info;
use blinklib::{
    hello_from_lib,
};

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max()); 
    let peripherals = esp_hal::init(config);
    let _io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();
    loop{
        info!("Hello.{}", hello_from_lib());
        delay.delay_millis(500);
    }

}

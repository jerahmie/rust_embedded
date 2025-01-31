#![no_std]
#![no_main]

use esp_backtrace as _;
use fugit::{HertzU32, Rate, RateExtU32,};
use esp_hal::{
    delay::Delay,
    main,
};
use esp_hal::ledc::{
    Ledc,
    LSGlobalClkSource,
    timer::{self, TimerIFace,},
    LowSpeed,
    channel::{self, ChannelIFace,},
};
    //gpio::{Level, Output},
use log::info;

const DUTY_LOW: u8 = 37;
const DUTY_HIGH: u8 = 67;

fn update_color(color: u32, ch: &mut channel::Channel<'_, LowSpeed>, delay: &Delay)->() {
    let mut mask: u32 = 0x8000_0000; 
    for _i in 1..32 {
        mask = mask/2;
        //info!("{:#b}", mask);
        if mask&color == 0 {
            let result = ch.set_duty(DUTY_LOW); 
            //info!("DUTY_LOW {:?}", result);
        } else {
            let result = ch.set_duty(DUTY_HIGH);
            //info!("DUTY_HIGH {:?}", result);
        }
    } 
        delay.delay_millis(1);
}

#[main]
fn main() -> ! {
    // generator version: 0.2.2

    // let _config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _period_hz: HertzU32 = Rate::<u32, 1, 1>::from_raw(42_000);
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Set GPIO8 as output and set state high initially
    // This is for ESP32-C3 Dev board
    //let mut led = Output::new(peripherals.GPIO8, Level::Low);
   
    let led = peripherals.GPIO8;
    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty5Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: 800.kHz(),
        }).unwrap();

    let mut channel0: channel::Channel<'_, LowSpeed>= ledc.channel(channel::Number::Channel0, led);
    channel0.configure(channel::config::Config { 
        timer: &lstimer0,
        duty_pct: 37,
        pin_config: channel::config::PinConfig::PushPull,
    }).unwrap(); 
    esp_println::logger::init_logger_from_env();
    let _ = channel0.set_duty(0);
    let delay = Delay::new();
    let c1 = 0xffff_ffff;
    loop {
        info!("Update LED");
        let _ = update_color(c1, &mut channel0, &delay);
        delay.delay_millis(501);
    }
}

#![no_std]
#![no_main]

use core::net::Ipv4Addr;
use embedded_io::*;
use esp_backtrace as _;
use esp_println::println;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::{
    timer::timg::TimerGroup,
    rng::Rng};
use log::info;
use esp_wifi::{
    init,
    wifi:: {
        utils::create_network_interface, AccessPointInfo, AuthMethod,
        ClientConfiguration, Configuration, WifiError, WifiStaDevice,
    },    
};
extern crate alloc;

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");


#[main]
fn main() -> ! {
    // generator version: 0.2.2

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_println::logger::init_logger_from_env();

    esp_alloc::heap_allocator!(72 * 1024);

    // wifi_init
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();
   
    let mut wifi = peripherals.WIFI;
    let (iface, device, mut controller) = 
        create_network_interface(&init, &mut wifi, WifiStaDevice).unwrap();

    let auth_method = AuthMethod::WPA2Personal;

    // client configuration 
    let client_config = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        password: PASSWORD.try_into().unwrap(),
        auth_method,
        ..Default::default()
    });

    let res = controller.set_configuration(&client_config);
    println!("Wi-Fi set_configuration returned {:?}", res);

    // Wi-Fi connect
    controller.start().unwrap();
    println!("Is wifi started: {:?}", controller.is_started());

    println!("Start Wifi scan");
    let res: Result<(heapless::Vec<AccessPointInfo, 10>, usize), WifiError> = controller.scan_n();
    if let Ok((res, _count)) = res {
        for ap in res {
            println!("{}: {}", ap.ssid, ap.signal_strength);
        }
    }
    let _res = controller.connect(); 
    let delay = Delay::new();
    println!("Entering main loop.");
    loop {
        //info!("Hello world!");
        //info!("WIFI State: {} {}", SSID, init.wifi());
        delay.delay_millis(1000);
        println!("Wait to get connected {:?}", controller.is_connected().unwrap());
    }


    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.23.1/examples/src/bin
}

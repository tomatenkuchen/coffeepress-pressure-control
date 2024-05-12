#![no_std]
#![no_main]

//use core::clone;

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{AdcConfig, Attenuation, ADC}, 
    clock::ClockControl, 
    delay::Delay, 
    gpio::IO, 
    peripherals::Peripherals, 
    prelude::*, 
};
use esp_println::println;

#[entry]
fn main() -> ! {
    // init peripherals access point
    let peripherals = Peripherals::take();
    // init system
    let system = peripherals.SYSTEM.split();
    // init clock
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let analog_pin = io.pins.gpio1.into_analog();

    // Create ADC instances
    let mut adc1_config = AdcConfig::new();
    let mut adc1_pin = adc1_config.enable_pin(analog_pin, Attenuation::Attenuation11dB);
    let mut adc1 = ADC::new(peripherals.ADC1, adc1_config);

    let mut delay = Delay::new(&clocks);

    loop {
        let pin_value: u16 = nb::block!(adc1.read(&mut adc1_pin)).unwrap();
        println!("ADC reading = {}", pin_value);
        delay.delay_ms(1000u32);
    }
}
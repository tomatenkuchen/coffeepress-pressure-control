#![no_std]
#![no_main]

extern crate lps28dfw;

use esp_backtrace as _;
use esp_hal::{
    analog::adc::{AdcConfig, Attenuation, Adc},
    clock::ClockControl,
    delay::Delay,
    gpio::Io,
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    // init peripherals access point
    let peripherals = Peripherals::take();
    // init system
    let system = SystemControl::new(peripherals.SYSTEM);
    // init clock
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let i2c_sda_pin = io.pins.gpio4;
    let i2c_scl_pin = io.pins.gpio5;

    // enable i2c for pressure sensor
    let i2c = I2C::new(
        peripherals.I2C0,
        i2c_sda_pin,
        i2c_scl_pin,
        100u32.kHz(),
        &clocks,
        None,
    );

    // set up pressure sensor
    let mut p_sens = lps28dfw::LPS28DFW::new(i2c, lps28dfw::I2CAddress::Low, lps28dfw::Range::Range4060hPa);
    // check if pressure sensor is alive
    p_sens.identify().unwrap();

    // get grid voltage input
    let mut adc1_config = AdcConfig::new();
    let mut adc_input_heater_current = adc1_config.enable_pin(io.pins.gpio0, Attenuation::Attenuation11dB);
    let mut adc_input_grid_voltage = adc1_config.enable_pin(io.pins.gpio1, Attenuation::Attenuation11dB);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    let delay = Delay::new(&clocks);

    loop {
        // read adc
        let i_grid_value: u16 = nb::block!(adc1.read_oneshot(&mut adc_input_heater_current)).unwrap();
        let v_grid_value: u16 = nb::block!(adc1.read_oneshot(&mut adc_input_grid_voltage)).unwrap();

        // read pressure sensor
        let p = p_sens.read_pressure().unwrap();
        println!("ADC1: v_grid = {}, i_grid = {}",v_grid_value, i_grid_value);
        println!("I2C0 : data array = {:?}", p);
        delay.delay_millis(2000u32);
    }
}

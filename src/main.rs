#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use atmega_hal::adc::{AdcSettings, ClockDivider, ReferenceVoltage};
use atmega_hal::clock::MHz10;
use atmega_hal::port::mode::{AnyInput, Floating, Input, Output};
use atmega_hal::port::{Pin, PB7, PC0, PC1, PD0, PD1};
use atmega_hal::usart::BaudrateExt;
use avr_device::atmega164pa::{TWI, USART0};
use core::panic::PanicInfo;
use embedded_hal::blocking::delay::DelayMs;
use mcp23017::PinMode::OUTPUT;

type Adc = atmega_hal::Adc<MHz10>;
type Delay = atmega_hal::delay::Delay<MHz10>;
type I2C = atmega_hal::I2c<MHz10>;
type MCP23017 = mcp23017::MCP23017<I2C>;
type Usart<USART, RX, TX> = atmega_hal::usart::Usart<USART, RX, TX, MHz10>;
type Usart0 = Usart<USART0, Pin<Input<AnyInput>, PD0>, Pin<Output, PD1>>;

fn delay_ms(ms: u16) {
    Delay::new().delay_ms(ms);
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

fn get_mcp23017(
    twi: TWI,
    i2c_scl: Pin<Input<Floating>, PC0>,
    i2c_sda: Pin<Input<Floating>, PC1>,
    mut mcp_rst: Pin<Output, PB7>,
) -> MCP23017 {
    let i2c = I2C::with_external_pullup(twi, i2c_sda, i2c_scl, 100000);

    mcp_rst.set_high();
    mcp_rst.set_low();
    delay_ms(50);
    mcp_rst.set_high();
    delay_ms(50);

    mcp23017::MCP23017::default(i2c).unwrap()
}

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let mut serial = Usart0::new(
        dp.USART0,
        pins.pd0,
        pins.pd1.into_output(),
        BaudrateExt::into_baudrate(1200u32),
    );

    ufmt::uwriteln!(serial, "starting\r").ok();

    let adc_settings = AdcSettings {
        clock_divider: ClockDivider::Factor16,
        ref_voltage: ReferenceVoltage::AVcc,
    };

    let mut adc = Adc::new(dp.ADC, adc_settings);

    let mut mcp = get_mcp23017(
        dp.TWI,
        pins.pc0.into_floating_input(),
        pins.pc1.into_floating_input(),
        pins.pb7.into_output(),
    );

    mcp.pin_mode(1, OUTPUT).unwrap();
    mcp.digital_write(1, true).unwrap();

    mcp.pin_mode(6, OUTPUT).unwrap();
    mcp.pull_up(6, true).unwrap();
    mcp.digital_write(6, true).unwrap();

    pins.pb2.into_output().set_high();

    let pin = pins.pa0.into_floating_input().into_analog_input(&mut adc);

    loop {
        let val = pin.analog_read(&mut adc);
        ufmt::uwriteln!(serial, "value: {}\r", val).ok();
        delay_ms(100);
    }
}

#![no_main]
#![no_std]

use cortex_m::{iprintln, asm};
use cortex_m_rt::entry;
use panic_itm as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*, adc, };

#[entry]
fn main() -> ! {
    //FREE IO 
    // A 1-4, 8-10, 15
    // B 0-2, 4-5, 8-15
    // C 0-13
    // D 0-15
    // E 6-7
    // F 2,4,6,9,10

    //Analog values 
    // 0 -> 0 mV
    // 2048 -> 1500 mV
    // 4095 -> 3000 mV


    let mut dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);
    let mut stim = &mut cortex_m::Peripherals::take().unwrap().ITM.stim[0];
    let mut adc1 = adc::Adc::adc1(
        dp.ADC1,
        &mut dp.ADC1_2,
        &mut rcc.ahb,
        adc::ClockMode::default(),
        clocks,
    );

    let mut pin0 = gpioa.pa1.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    // let mut temperature: i32 = -1;
    // let mut light_level: i32 = -1;
    // let mut humidity_level: u8 = 0;

    loop {
        let pin0_data: u16 = adc1.read(&mut pin0).expect("Error reading adc1.");
        iprintln!(stim, "result {}", map_adc_light_percent(pin0_data) );
        asm::delay(1_000_000);
    }
}

// light sensor goes from 3700 adc to 4080
// Lower value after map means the room is brighter
fn map_adc_light_percent(data: u16) -> u16{

    let difference: u16 = 4096 as u16 - data;
    let result: u16 = 100 - (difference * 100 / 396);

    return result;
}
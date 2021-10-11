#![no_main]
#![no_std]

use cortex_m::{iprintln, asm};
use cortex_m_rt::entry;
use panic_itm as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*, adc, timer::Timer };
use numtoa::{self, NumToA};
use lcd1602::LCD1602;
use dht_sensor::*;
// use dht11::Dht11;

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

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut adc1 = adc::Adc::adc1(
        dp.ADC1,
        &mut dp.ADC1_2,
        &mut rcc.ahb,
        adc::ClockMode::default(),
        clocks,
    );
    let mut buf = [0u8; 20];

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiod = dp.GPIOD.split(&mut rcc.ahb);

    let mut pin1 = gpioa.pa1.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);
    let mut pin2 = gpioa.pa2.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut temperature_level: i8 = 0;
    let mut light_level: u16 = 0;
    let mut humidity_level: u8 = 0;


    // Init pins
    let mut rs = gpiod.pd1.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut en = gpiod.pd2.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut d4 = gpiod.pd3.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut d5 = gpiod.pd4.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut d6 = gpiod.pd5.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut d7 = gpiod.pd6.into_push_pull_output(&mut gpiod.moder, &mut gpiod.otyper);
    let mut timer = Timer::new(dp.TIM6, clocks, &mut rcc.apb1);
    
    let mut lcd = LCD1602::new(en, rs, d4, d5, d6, d7, timer).unwrap();

    {
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
        delay.delay_ms(1000_u16);
        cp.SYST = delay.free();
    }

    loop {

        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
        match dht11::Reading::read(&mut delay, &mut pin2) {
            Ok(dht11::Reading {
                temperature,
                relative_humidity,
            }) => {
                cp.SYST = delay.free();
                iprintln!(&mut cp.ITM.stim[0], "Temp: {}", temperature );
                iprintln!(&mut cp.ITM.stim[0], "Humidity: {}", relative_humidity );
                temperature_level = temperature;
                humidity_level = relative_humidity;
            },
            Err(e) => {
                cp.SYST = delay.free();
                iprintln!(&mut cp.ITM.stim[0], "Error" );
            },
        }

        let pin1_data: u16 = adc1.read(&mut pin1).expect("Error reading adc1.");
        light_level = map_adc_light_percent(pin1_data);
        iprintln!(&mut cp.ITM.stim[0], "Light: {}", light_level );

        //Show user feedback
        lcd.clear();
        lcd.print("L:");
        lcd.print(light_level.numtoa_str(10, &mut buf));
        lcd.print(" T:");
        lcd.print(temperature_level.numtoa_str(10, &mut buf));
        lcd.print(" H:");
        lcd.print(humidity_level.numtoa_str(10, &mut buf));

        
        asm::delay(10_000_000);
    }
}

// light sensor goes from 3700 adc to 4080
// Lower value after map means the room is brighter
fn map_adc_light_percent(data: u16) -> u16{
    if data > 4095 || data < 3700{
        return 0;
    }else{
        let difference: u16 = 4096 as u16 - data;
        let result: u16 = 100 - (difference * 100 / 396);
        return result;
    }
}
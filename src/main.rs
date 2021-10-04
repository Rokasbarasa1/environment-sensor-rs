#![no_main]
#![no_std]

// use aux8::entry;
// use cortex_m;
use cortex_m::{iprintln, asm};
use cortex_m_rt::entry;
// use stm32f3_discovery;
use stm32f3xx_hal;
// use f3::hal::stm32f30x::{GPIOA, RCC};
// use cortex_m_semihosting::hprintln;
// #[allow(unused_imports)]
// use aux6::{entry, iprint, iprintln};
use panic_itm as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let mut pin = gpioa.pa8.into_pull_down_input(&mut gpioa.moder, &mut gpioa.pupdr);
    let p = pac::ITM::take().unwrap();
    let mut itm = p.ITM;
    loop {
        if pin.is_high().unwrap() {
            iprintln!(&mut itm.stim[0], "Hello, world!");
        }else{
            iprintln!(&mut itm.stim[0], "NULL WORLD");
        }
        // led.toggle().unwrap();
        asm::delay(8_000_000);
    }
    

    // let mut temperature: i32 = -1;
    // let mut light_level: i32 = -1;
    // let mut humidity_level: u8 = 0;

    //FREE IO 
    // A 1-4, 8-10, 15
    // B 0-2, 4-5, 8-15
    // C 0-13
    // D 0-15
    // E 6-7
    // F 2,4,6,9,10
    // Turn on GPIOA_8 port
    // let p = stm32f3xx_hal
    // stm32f3xx_hal::gpio::Gpioa
    // let gpioa = GPIOA::ptr();
    // let rcc = RCC::ptr();
    // (*rcc).ahbenr.write(|w| w.iopaen().set_bit());

    // (*gpioa).moder.write(|w| {
    //     w.moder8().input()
    // });

    // GPIOx_MODER - modify port
    // GPIOx_OTYPER and GPIOx_OSPEEDR registers are used to select the output type (push-pull or open-drain) and speed
    // asm::bkpt();

    // loop {

    // }
}
use crate::error::Error;
use crate::LCD1602;

use crate::error::Error::UnsupportedBusWidth;
use crate::lcd1602::BusWidth::FourBits;
use crate::lcd1602::Direction::RightToLeft;
// use core::time::Duration;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::timer::CountDown;
use embedded_time;
use embedded_time::duration::Microseconds;
use nb::block;

impl<EN, RS, D4, D5, D6, D7, Timer, E> LCD1602<EN, RS, D4, D5, D6, D7, Timer>
where
    EN: OutputPin<Error = E>,
    RS: OutputPin<Error = E>,
    D4: OutputPin<Error = E>,
    D5: OutputPin<Error = E>,
    D6: OutputPin<Error = E>,
    D7: OutputPin<Error = E>,
    Timer: CountDown<Time = embedded_time::duration::Generic<u32>>,
{
    pub fn new(
        en: EN,
        rs: RS,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        timer: Timer,
    ) -> Result<LCD1602<EN, RS, D4, D5, D6, D7, Timer>, Error<E>> {
        let mut lcd = LCD1602 {
            en,
            rs,
            d4,
            d5,
            d6,
            d7,
            timer,
            char_count: 0
        };
        lcd.init()?;
        Ok(lcd)
    }

    fn init(&mut self) -> Result<(), Error<E>> {
        self.delay(50000)?;
        self.set_bus_width(FourBits)?;

        self.command(0x0C)?; // Display mode
        self.clear()?;
        self.set_entry_mode(RightToLeft, false)?;
        Ok(())
    }

    pub fn set_bus_width(&mut self, bus_width: BusWidth) -> Result<(), Error<E>> {
        match bus_width {
            FourBits => {
                self.write_bus(0x02)?;
                self.delay(39)
            }
            _ => Err(UnsupportedBusWidth),
        }
    }
    pub fn set_entry_mode(
        &mut self,
        text_direction: Direction,
        screen_edge_tracking: bool,
    ) -> Result<(), Error<E>> {
        let mut cmd = 0x04;
        if text_direction == Direction::RightToLeft {
            cmd |= 0x02;
        }
        if screen_edge_tracking {
            cmd |= 0x01;
        }
        self.command(cmd)?;
        self.delay(39)
    }

    pub fn clear(&mut self) -> Result<(), Error<E>> {
        self.command(0x01)?;
        self.char_count = 0;
        self.delay(1530)
    }

    pub fn home(&mut self) -> Result<(), Error<E>> {
        self.command(0x02)?;
        self.delay(1530)
    }

    fn command(&mut self, cmd: u8) -> Result<(), Error<E>> {
        self.rs.set_low()?;
        self.write_bus((cmd & 0xF0) >> 4)?;
        self.write_bus(cmd & 0x0F)?; // 4bit writes send end pulses
        Ok(())
    }

    fn write_char(&mut self, ch: u8) -> Result<(), Error<E>> {
        self.rs.set_high()?;
        self.write_bus((ch & 0xF0) >> 4)?;
        self.write_bus(ch & 0x0F)?; // 4bit writes send end pulses
        Ok(())
    }

    pub fn next_line(&mut self) -> Result<(), Error<E>> {
        self.rs.set_low()?;
        self.en.set_low()?;
        self.command(0x0C)?;
        self.char_count = 0;
        self.delay(1530)
    }
    //Will not print characters if too many
    pub fn print(&mut self, s: &str) -> Result<(), Error<E>> {
        // if self.char_count != 16 {
            for ch in s.chars() {
                // if self.char_count == 7{
                //     self.next_line();
                // }else if self.char_count == 16 {
                //     break;
                // }

                self.delay(320)?; // per char delay
                self.write_char(ch as u8)?;
                // self.char_count = self.char_count + 1;
            }
        // }
        
        Ok(())
    }

    pub fn println(&mut self, s: &str) -> Result<(), Error<E>> {
        if self.char_count != 15 {
            for ch in s.chars() {
            
                self.delay(320)?; // per char delay
                self.write_char(ch as u8)?;
                self.char_count = self.char_count + 1;
            }
        }
        
        Ok(())
    }

    fn write_bus(&mut self, data: u8) -> Result<(), Error<E>> {
        self.en.set_low()?;
        match (data & 0x1) > 0 {
            true => self.d4.set_high()?,
            false => self.d4.set_low()?,
        };
        match (data & 0x2) > 0 {
            true => self.d5.set_high()?,
            false => self.d5.set_low()?,
        };
        match (data & 0x4) > 0 {
            true => self.d6.set_high()?,
            false => self.d6.set_low()?,
        };
        match (data & 0x8) > 0 {
            true => self.d7.set_high()?,
            false => self.d7.set_low()?,
        };
        self.en.set_high()?;
        self.en.set_low()?;
        Ok(())
    }

    pub fn delay(&mut self, interval_us: u32) -> Result<(), Error<E>> {
        self.timer.start(Microseconds(interval_us as u32));
        match block!(self.timer.wait()) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::TimerError),
        }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
}

#[derive(PartialEq)]
pub enum BusWidth {
    FourBits,
    EightBits,
}

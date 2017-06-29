//TODO: somehow start the timer
//TODO: print when timer expires
//TODO: tofgle LED when timer expieres

// use core::cell::Cell;
// use kernel::{AppId, Driver, AppSlice, Shared, Container};
// use kernel::common::take_cell::TakeCell;
// extern crate nrf51;
// use boards::nrf51dk;
use kernel::hil;
//use kernel::hil::uart;
// use kernel::hil::time::Frequency;
// use kernel::process::Error;
// use kernel::returncode::ReturnCode;
// use radio;


pub struct PlayMe<'a, A: hil::time::Alarm + 'a> {
    alarm: &'a A,
}

impl<'a, A: hil::time::Alarm + 'a> PlayMe<'a, A> {
    pub fn new(alarm: &'a A) -> PlayMe<'a, A> {
        PlayMe {
            alarm: alarm,
        }
     }

    pub fn configure_periodic_alarm(&self) {
        let interval_in_tics = self.alarm.now().wrapping_add(100000);
        self.alarm.set_alarm(interval_in_tics);

    }
}


impl<'a, A: hil::time::Alarm + 'a> hil::time::Client for PlayMe<'a, A> {
    fn fired(&self) {
        self.configure_periodic_alarm();
        // println!("Done scanning for I2C devices. Buffer len:");
         debug!("Initialization complete. Entering main loop");


    }
}

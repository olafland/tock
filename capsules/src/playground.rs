// toggle LEDS in some funny patterns
// read some sensor
// wait for button pressed

//#![feature(const_fn, const_cell_new)]
//#![no_std]

#[allow(unused_imports)]
//#[macro_use(debug)]
//extern crate kernel;

//use kernel::hil::sensors::{AmbientLight, AmbientLightClient};
use kernel::hil::time::{self, Alarm, Frequency};

pub struct Playground<'a, A: Alarm + 'a> {
    alarm: &'a A,
    //    light: &'a AmbientLight,
}

impl<'a, A: Alarm> Playground<'a, A> {
    pub fn new(alarm: &'a A /*, light: &'a AmbientLight*/) -> Playground<'a, A> {
        Playground {
            alarm: alarm,
            //light: light,
        }
    }

    pub fn start(&self) {
        debug!("Blink Capsule for NRF5x starting");
        //TODO: need so switch to real time unit
        //TODO: can we also have reoccuring timers?
        self.alarm.set_alarm(self.alarm.now().wrapping_add(10000));
    }
}

impl<'a, A: Alarm> time::Client for Playground<'a, A> {
    fn fired(&self) {
        let t0 = self.alarm.now();
        self.alarm.set_alarm(t0.wrapping_add(10000));
        debug!("Alarm fired: {}", t0);
    }
}

// impl<'a, A: Alarm> AmbientLightClient for Playground<'a, A> {
//     fn callback(&self, lux: usize) {}
// }

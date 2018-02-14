//TODO: tofgle LED when timer expieres

// use core::cell::Cell;
// use kernel::{AppId, Driver, AppSlice, Shared, Container};
// use kernel::common::take_cell::TakeCell;
// extern crate nrf51;
// use boards::nrf51dk;
use kernel::hil::time::{Alarm, Client};
//use kernel::hil::uart;
// use kernel::hil::time::Frequency;
// use kernel::process::Error;
// use kernel::returncode::ReturnCode;
// use radio;

pub struct PlayMe<'a, A: Alarm + 'a> {
    alarm: &'a A,
}

// pub static mut PLAYME: PlayMe = PlayMe::new(alarm);

// let xmac_alarm = static_init!(
//         VirtualMuxAlarm<'static, sam4l::ast::Ast>,
//         VirtualMuxAlarm::new(mux_alarm));

//     let xmac: &XMacDevice =
//         static_init!(XMacDevice, xmac::XMac::new(rf233, xmac_alarm, &sam4l::trng::TRNG));

impl<'a, A: Alarm + 'a> PlayMe<'a, A> {
    pub fn new(alarm: &'a A) -> PlayMe<'a, A> {
        PlayMe { alarm: alarm }
    }

    pub fn configure_periodic_alarm(&self) {
        debug!("alarm starting");
        let interval_in_tics = self.alarm.now().wrapping_add(100000);
        self.alarm.set_alarm(interval_in_tics);
    }
}

impl<'a, A: Alarm + 'a> Client for PlayMe<'a, A> {
    fn fired(&self) {
        self.configure_periodic_alarm();
        // println!("Done scanning for I2C devices. Buffer len:");
        debug!("play timer fired");
    }
}

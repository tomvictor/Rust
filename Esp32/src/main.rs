use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::hal::{
    delay::FreeRtos,
    gpio::{InterruptType, PinDriver, Pull, Level},
    task::notification::Notification,
};
use std::{thread, time};
use std::time::Duration;
use log::log;

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();


    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio7).unwrap();

    log::info!("spawn led thread...");
    let thread_handle = thread::spawn(move || {
        loop {
            log::info!("toggle led...");
            thread::sleep(Duration::from_millis(500));
            led.toggle().unwrap();
        }
    });

    thread_handle.join().expect("TODO: panic message");
}

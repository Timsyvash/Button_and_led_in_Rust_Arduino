#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut count = 0;
    let mut last_button_state = false;

    let mut led = pins.d10.into_output();
    let button = pins.d3.into_pull_up_input();

    loop {
        let button_state = button.is_high();

        if button_state && !last_button_state {
            count += 1;
        }

        last_button_state = button_state;

        if count % 2 == 0 {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}

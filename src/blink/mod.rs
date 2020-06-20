extern crate battery;

use battery::{Battery, State};
use std::i16;

/// FSM for next blink value by rotating {0, 1, custom, terminal-width}.
pub fn cycle(blink_max: u16, blink_custom: u16, size: u16) -> u16 {
    match (blink_custom as i16 - blink_max as i16).signum() {
        -1 => 0,
        0 => size,
        _ => blink_custom,
    }
}

pub fn increment(batt: &Battery, blink: u16, blink_max: u16) -> u16 {
    if batt.state() != State::Charging || blink >= blink_max {
        return 0;
    }

    blink + 1
}

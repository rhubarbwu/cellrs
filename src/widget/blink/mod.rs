extern crate battery;

use battery::{Battery, State};
use std::i16;

pub struct BlinkState {
    val: u16,
    max: u16,
    custom: u16,
    reset: bool,
}

impl BlinkState {
    /// Internal constructor with default values.
    pub fn new() -> BlinkState {
        BlinkState {
            val: 0,
            max: 1,
            custom: 1,
            reset: false,
        }
    }

    /// FSM for custom blink value, rotating {0, custom, terminal-width}.
    pub fn cycle(&mut self, size: u16) {
        self.max = match (self.custom as i16 - self.max as i16).signum() {
            -1 => 0,
            0 => size,
            _ => self.custom,
        }
    }

    /// FSM for resetting or incrementing the blink value.
    pub fn increment(&mut self, batt: &Battery) {
        if batt.state() != State::Charging {
            self.val = 0;
            return;
        }
        if self.reset || self.val >= self.max {
            self.val = 0;
            self.reset = false;
        } else {
            self.val += 1;
        }
    }

    /// Getter function for value.
    pub fn get_value(&self) -> u16 {
        self.val
    }

    /// Setter function for max and custom max.
    pub fn set_max(&mut self, new_max: u16) {
        self.max = new_max;
        self.custom = new_max;
    }

    /// Setter function for reset.
    pub fn set_reset(&mut self) {
        self.reset = true;
    }
}

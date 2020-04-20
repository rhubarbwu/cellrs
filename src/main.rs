extern crate battery;
extern crate chrono;
extern crate termion;

mod display;

use battery::{Manager, State};
use chrono::prelude::*;
use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;
use termion::{async_stdin, clear, cursor, raw::IntoRawMode};

const ASCII_ESC: u8 = 27;
const ASCII_Q: u8 = 113;
const REFRESH: Duration = Duration::from_millis(100);

fn main() -> Result<(), battery::Error> {
    // Battery manager and index of selected battery (default 0).
    let manager = Manager::new()?;
    let index = 0;

    // Initialize the IO and clear the terminal.
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "\n{}{}\n", cursor::Hide, clear::All).unwrap();

    // Initialize the battery settings and registers.
    let blink_max = 1;
    let blink_on = true;
    let mut blink = 0;
    let mut force = true;
    let mut level = 101 as u16;
    let mut state = State::Unknown;

    // Set up the time/clock format and refresh.
    let format = "%H:%M:%S".to_string();
    let clock: &str = format.as_str();
    loop {
        // Get selected battery.
        let battery = match manager.batteries()?.nth(index) {
            None => break,
            Some(maybe_batt) => match maybe_batt {
                Err(_) => break,
                Ok(batt) => batt,
            },
        };

        // If the battery has changed level or state, display.
        if force {
            blink = display::display_battery(&mut stdout, &battery, blink);
            if blink > blink_max {
                blink = 0;
            }
        }

        // Wait until the next clock cycle, then refresh.
        // Refresh early if terminal size or battery level/state change.
        let mut exit = false;
        let time = Local::now().format(clock).to_string();
        let size = termion::terminal_size().unwrap();
        while Local::now().format(clock).to_string() == time {
            // Match user use input to keypress functions.
            if let Some(Ok(b)) = stdin.next() {
                match b {
                    ASCII_ESC | ASCII_Q => {
                        exit = true;
                        break;
                    }
                    _ => (),
                }
            }
            thread::sleep(REFRESH);

            // Check if the terminal size or battery level/state changed.
            // If not, loop and wait for the next clock tick to refresh.
            // If so, update the changed value force an refresh.
            force = true;
            if size != termion::terminal_size().unwrap() {
                write!(stdout, "{}", clear::All).unwrap();
            } else if level != display::battery_level(&battery) {
                level = display::battery_level(&battery);
            } else if state != battery.state() {
                state = battery.state();
            } else {
                force = blink_on;
            }
        }

        // If the refresh resulted from the user quitting, break out of loop.
        if exit {
            break;
        }
    }

    // Reset prompt position before exiting.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    Ok(())
}

extern crate battery;
extern crate chrono;
extern crate termion;

mod display;

use chrono::prelude::*;
use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;
use termion::{async_stdin, clear, cursor, raw::IntoRawMode};

const REFRESH: Duration = Duration::from_millis(100);

fn main() -> Result<(), battery::Error> {
    // The index of the selected battery.
    let index = 0;

    // Set up the time/clock format and refresh.
    let format = "%H:%M:%S".to_string();
    let clock: &str = format.as_str();

    // Initialize the IO and size of the terminal.
    let mut size = termion::terminal_size().unwrap();
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout().into_raw_mode().unwrap();

    loop {
        // Reset display position.
        write!(stdout, "\n{}{}\n", cursor::Hide, clear::All).unwrap();

        // Display the selected battery.
        let manager = battery::Manager::new()?;
        for (idx, maybe_batt) in manager.batteries()?.enumerate().next() {
            let battery = maybe_batt?;
            display::display_battery(&battery, &mut stdout);
            if idx >= index {
                break;
            }
        }

        // Wait until the next clock cycle, then refresh.
        // Refresh early on appropriate user input or terminal resize.
        let mut exit = 0;
        let time = Local::now().format(clock).to_string();
        while time == Local::now().format(clock).to_string() {
            let ev = stdin.next();
            if let Some(Ok(b)) = ev {
                match b {
                    b'q' => {
                        exit = 1;
                        break;
                    }
                    _ => (),
                }
            }

            // Check for a terminal resize, and refresh on resize.
            if display::check_resize(size, &mut stdout) {
                size = termion::terminal_size().unwrap();
                break;
            }
            thread::sleep(REFRESH);
        }

        // If the refresh resulted from the user quitting, break out of loop.
        if exit == 1 {
            break;
        }
    }

    // Reset prompt position before exiting.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    Ok(())
}

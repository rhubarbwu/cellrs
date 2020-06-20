extern crate battery;
extern crate chrono;
extern crate termion;

mod blink;
mod display;
mod help;

use battery::Manager;
use chrono::prelude::*;
use std::io::{stdout, Read, Write};
use std::time::Duration;
use std::{env, thread};
use termion::{async_stdin, clear, cursor, raw::IntoRawMode};

const ASCII_ESC: u8 = 27;
const ASCII_B: u8 = 98;
const ASCII_Q: u8 = 113;
const REFRESH: Duration = Duration::from_millis(100);

/// Main function for cellrs, including argument processing and display loop.
fn main() -> Result<(), battery::Error> {
    // Initialize the battery settings and registers.
    let mut blink_custom = 1;
    let mut blink_max = 1;
    let mut blink = 0;

    // Process command-line arguments and exit to help if specified.
    let args: Vec<String> = env::args().collect();
    let name = String::from(&args[0].to_string());
    for i in 1..args.len() {
        match args[i].as_str() {
            "-b" => {
                // If the -b blink flag is used but no value is specified.
                if args.len() <= i + 1 {
                    blink_max = 1; // Default blink-value to 1.
                    continue;
                }

                // Atempt parsing blink-value to u16.
                let ch = String::from(&args.get(i + 1).unwrap().to_string());
                match ch.parse::<u8>() {
                    Ok(m) => {
                        blink_custom = m as u16;
                        blink_max = blink_custom;
                    }
                    _ => {
                        blink_max = 1;
                    }
                }
            }
            "-h" => {
                help::print(&name);
            }
            _ => {}
        }
    }

    // Battery manager and index of selected battery (default 0).
    let manager = Manager::new()?;
    let index = 0;

    // Initialize the IO and clear the terminal.
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "\n{}{}\n", cursor::Hide, clear::All).unwrap();

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
        display::display_battery(&mut stdout, &battery, blink);

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
                    ASCII_B => {
                        blink_max = blink::cycle(blink_max, blink_custom, size.0);
                    }
                    _ => (),
                }
            }
            thread::sleep(REFRESH);

            // Clear the screen if the terminal size changed.
            if size != termion::terminal_size().unwrap() {
                write!(stdout, "{}", clear::All).unwrap();
            }
        }

        // If the refresh resulted from the user quitting, break out of loop.
        if exit {
            break;
        }

        blink = blink::increment(&battery, blink, blink_max);
    }

    // Reset prompt position before exiting.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    Ok(())
}

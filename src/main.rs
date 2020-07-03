extern crate battery;
extern crate chrono;
extern crate termion;

mod ascii_keys;
mod help;
mod widget;

use ascii_keys::*;
use battery::Manager;
use chrono::prelude::Local;
use std::io::{stdout, Read, Write};
use std::{env::args, thread::sleep, time::Duration};
use termion::{async_stdin, clear, cursor, raw::IntoRawMode};
use widget::blink::BlinkState;

const REFRESH: Duration = Duration::from_millis(100);

fn preprocess() -> (BlinkState, usize) {
    // Initialize the battery blink level, max, custom max, reset flag.
    let mut blink_inst = BlinkState::new();
    let mut index = 0;

    // Process command-line arguments and exit to help if specified.
    let args: Vec<String> = args().collect();
    let name = String::from(&args[0].to_string());
    for i in 1..args.len() {
        match args[i].as_str() {
            "-b" => {
                // If the -b blink flag is used but no value is specified.
                if args.len() <= i + 1 {
                    continue;
                }

                // Attempt parsing blink-value to u16.
                let ch = String::from(&args.get(i + 1).unwrap().to_string());
                match ch.parse::<u8>() {
                    Ok(m) => blink_inst.set_max(m as u16),
                    _ => {}
                }
            }
            "-h" => help::print(&name),
            "-1" => index = 0,
            "-2" => index = 1,
            "-3" => index = 2,
            "-4" => index = 3,
            _ => {}
        }
    }

    (blink_inst, index)
}

/// Main function for cellrs, including argument processing and display loop.
fn main() -> Result<(), battery::Error> {
    // Initialize the battery blink level, max, custom max, reset flag.
    let (mut blink_inst, mut index) = preprocess();

    // Initialize the IO and clear the terminal.
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "\n{}{}\n", cursor::Hide, clear::All).unwrap();

    // Battery manager and index of selected battery (default 0).
    let manager = Manager::new()?;

    // Set up the time/clock format and refresh.
    let format = "%H:%M:%S".to_string();
    let clock: &str = format.as_str();
    loop {
        // Get selected battery.
        let battery = match (index, manager.batteries()?.nth(index)) {
            (0, None) => break,          // 0th battery doesn't work. Break.
            (_, Some(Ok(batt))) => batt, // Working battery stats.
            _ => {
                // Indexed battery doesn't exist; index to 0 and try again.
                index = 0;
                continue;
            }
        };

        // If the battery has changed level or state, display.
        widget::display_battery(&mut stdout, &battery, &mut blink_inst);

        // Wait until the next clock cycle, then refresh.
        // Refresh early if terminal size or battery level/state change.
        let mut exit = false;
        let time = Local::now().format(clock).to_string();
        let size = termion::terminal_size().unwrap();
        while Local::now().format(clock).to_string() == time {
            // Match user input to keypress functions.
            if let Some(Ok(b)) = stdin.next() {
                match b {
                    KEY_ESC | KEY_Q => {
                        exit = true;
                        break;
                    }
                    KEY_B => blink_inst.cycle(size.0),
                    NUM_1..=NUM_4 => index = (b - NUM_1) as usize,
                    _ => (),
                }
            }
            sleep(REFRESH);

            // Clear the screen if the terminal size changed.
            if size != termion::terminal_size().unwrap() {
                write!(stdout, "{}", clear::All).unwrap();
            }
        }

        // If the refresh resulted from the user quitting, break out of loop.
        if exit {
            break;
        }

        blink_inst.increment(&battery);
    }

    // Reset prompt position before exiting.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    Ok(())
}

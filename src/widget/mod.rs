extern crate battery;
extern crate termion;

pub mod blink;

use battery::units::ratio::percent;
use battery::Battery;
use blink::BlinkMoment;
use std::io::Write;
use termion::{color, cursor::Goto, raw::RawTerminal as RawTerm};

// Visual characters for battery.
const CELL_BLANK: &str = " ";
const CELL_CHAR: &str = "|";
const CELL_WALL: &str = "=";

const DIV: u16 = 5;

/// Returns battery charge level percentage as u16.
pub fn battery_level(batt: &Battery) -> u16 {
    batt.state_of_charge().get::<percent>().round() as u16
}

/// Returns battery height/width based on dimensions of the terminal.
/// - The sizes used in pattern matching are to some degree arbitrary.
/// - Returns (0,0) when the terminal is too thin or short: < (10, 7).
///   - This will disallow the next refresh, but allows recovery and refresh
///     after next acceptable resize.
fn battery_size() -> (u16, u16) {
    let (term_width, term_height) = termion::terminal_size().unwrap();

    /*  Round the width down to the next multiple of 5 and subtract the
    minimum of the last pattern. */
    let x = match term_width {
        0..=9 => return (0, 0),
        10..=24 => (term_width / DIV - 1),
        25..=49 => (term_width / DIV - 2),
        50..=99 => (term_width / DIV - 5),
        _ => (term_width / DIV - 10),
    } * DIV;

    // Truncate the height to an appropriate value, to include the stats.
    let y = match term_height {
        0..=6 => return (0, 0),
        7..=10 => term_height - 4,
        11..=25 => 7,
        26..=50 => 8,
        51..=100 => 9,
        _ => 10,
    };
    (x, y)
}

/// Return position of the top-left corner of the battery.
fn battery_top_left() -> (u16, u16) {
    let (cent_x, cent_y) = terminal_centre();
    let (size_x, size_y) = battery_size();

    (cent_x - size_x / 2 + 1, cent_y - size_y / 2)
}

/// Default red-yellow-green colour theme for the battery cells.
fn cell_colour(x: u8, x_size: u8) -> u8 {
    match x / (x_size / 5) {
        0 => 9,      // Red.
        1..=2 => 11, // Yellow.
        3..=4 => 10, // Green.
        _ => 0,      // Black. This shouldn't happen.
    }
}

/// Display a battery in the centre of the terminal.
/// - The dimensions of the battery scale with the terminal.
/// - The status and percentage are also shown.
/// - Blinking cells are shown, and the blink counter <blink> is updated.
/// - Early-return if the battery size (based on terminal size) is too small.
pub fn display_battery<W: Write>(
    out: &mut RawTerm<W>,
    batt: &Battery,
    blink_moment: &mut BlinkMoment,
) {
    let (batt_width, batt_height) = match battery_size() {
        (0, 0) => return,
        (bw, bh) => (bw, bh),
    };
    let perc = battery_level(batt);
    let pos = battery_top_left();

    // Iterate through width/height to print the battery walls/cells.
    for x in 0..batt_width {
        for y in 0..batt_height {
            let blink = blink_moment.get_value();
            // Get the fill character and colour based on position and blink.
            let (fill, color) = if y == 0 || batt_height - y == 1 {
                (CELL_WALL, 15) // White cell wall.
            } else if x >= blink && 100 * (x - blink) > perc * (batt_width) {
                (CELL_BLANK, 0) // Black blank.
            } else if 100 * x > perc * batt_width {
                if x + 1 >= batt_width {
                    blink_moment.set_reset();
                }
                // Cyan blinking cell.
                (CELL_CHAR, 14)
            } else {
                // Regular coloured cell.
                (CELL_CHAR, cell_colour(x as u8, batt_width as u8))
            };

            // Set colour and write the cell/wall to the terminal.
            let goto_pos = Goto(pos.0 + x, pos.1 + y);
            let colour = color::Fg(color::AnsiValue(color));
            write!(out, "{}{}{}", goto_pos, colour, fill).unwrap();
        }
    }

    // Set status position and colourm, then print them under the battery.
    let stat_pos = Goto(pos.0, pos.1 + batt_height + 1);
    let white = color::Fg(color::White);
    write!(out, "{}{}", stat_pos, " ".repeat(batt_width as usize)).unwrap();
    write!(out, "{}{}{}% - {}", stat_pos, white, perc, batt.state()).unwrap();

    // Flush the output stream and return the new blink counter.
    out.flush().unwrap();
}

/// Return the centre position of the terminal.
fn terminal_centre() -> (u16, u16) {
    let (x, y) = termion::terminal_size().unwrap();
    (x / 2, y / 2)
}

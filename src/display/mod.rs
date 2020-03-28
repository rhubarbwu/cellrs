extern crate battery;
extern crate termion;

use battery::units::ratio::percent;
use battery::Battery;
use std::io::Write;
use termion::{clear, color, cursor, raw::RawTerminal as Term};

// Visual characters for battery.
const CELL_CHAR: &str = "|";
const CELL_WALL: &str = "=";

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
        10..=24 => (term_width / 5) * 5 - 5,
        25..=49 => (term_width / 5) * 5 - 10,
        50..=99 => (term_width / 5) * 5 - 25,
        _ => (term_width / 5) * 5 - 50,
    };

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

/// CHeck if the terminal has been resized.
pub fn check_resize<W: Write>(size: (u16, u16), out: &mut Term<W>) -> bool {
    if size == termion::terminal_size().unwrap() {
        return false;
    }
    write!(out, "{}", clear::All).unwrap();
    true
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
/// - Early-return if the battery size (based on terminal size) is too small.
pub fn display_battery<W: Write>(b: &Battery, out: &mut Term<W>) {
    let (b_width, b_height) = match battery_size() {
        (0, 0) => return,
        (bw, bh) => (bw, bh),
    };
    let perc = b.state_of_charge().get::<percent>().round() as u16;
    let pos = top_left();

    // Iterate through the width of the battery.
    for x in 0..b_width {
        // Iterate through the height to print the walls and cells.
        for y in 0..b_height {
            let (fill, color) = match (y, b_height - y) {
                (0, _) | (_, 1) => (CELL_WALL, 15),
                // Skip this cell if it's beyond the battery's percentage.
                _ => match 100 * x > perc * b_width {
                    true => continue,
                    _ => (CELL_CHAR, cell_colour(x as u8, b_width as u8)),
                },
            };

            // Write the cell or wall to the terminal.
            write!(
                out,
                "{}{}{}",
                cursor::Goto(pos.0 + x, pos.1 + y),
                color::Fg(color::AnsiValue(color)),
                fill,
            )
            .unwrap();
        }
    }

    // Set the position for the status and percentage line.
    let stat_pos = cursor::Goto(pos.0, pos.1 + b_height + 1);
    let white = color::Fg(color::White);
    write!(out, "{}{}{}% - {}", stat_pos, white, perc, b.state()).unwrap();
    out.flush().unwrap();
}

/// Return the centre position of the terminal.
fn terminal_centre() -> (u16, u16) {
    let (x, y) = termion::terminal_size().unwrap();
    (x / 2, y / 2)
}

/// Return position of the top-left corner of the battery.
fn top_left() -> (u16, u16) {
    let (cent_x, cent_y) = terminal_centre();
    let (size_x, size_y) = battery_size();

    (cent_x - size_x / 2, cent_y - size_y / 2)
}

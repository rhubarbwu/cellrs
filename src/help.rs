use std::process;

const ARGUMENTS: &str = "
    Command-Line Arguments

    -b [width]  Set custom blink-width [16-bit unsigned] (defaults to 1).
    -h          Display this help message.
    -[1-4]      Select battery index. The right-most option will override.
                Index-out-of-bounds will default index to 0.";

const KEY_COMMANDS: &str = "
    Key Commands

    b       Cycle through the blink-width value.
                If your custom blink-width was 1 or unset, it will cycle
                through {1, <max-width>, 0, ...}. Otherwise, it will
                cycle through {<custom-width>, <max-width>, 0, ...}.
    q       Quit cellrs.
    [1-4]   Switch between up to the first four indexed batteries.
                Index-out-of-bounds will default index to 0.";

/// Prints the help message.
pub fn print(name: &String) {
    println!("usage : {}\n{}\n{}", name, ARGUMENTS, KEY_COMMANDS);
    process::exit(0);
}

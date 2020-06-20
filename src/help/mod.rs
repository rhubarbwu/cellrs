use std::process;

/// Prints the help message.
pub fn print(name: &String) {
    println!("usage : {}", name);
    println!("\t-b\tSet custom blink width [16-bit unsigned] (defaults to 1).");
    println!("\t-h\tDisplay this help message.");
    process::exit(1);
}

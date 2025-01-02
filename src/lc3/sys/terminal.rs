use std::{
    error::Error,
    io::{Read, Write},
    process,
    thread,
};

use libc::STDIN_FILENO;
use signal_hook::{iterator::Signals, SIGINT};
use termios::{
    cfmakeraw, tcsetattr, Termios, TCSANOW,
};


pub fn turn_off_canonical_and_echo_modes() -> Result<(), Box<dyn Error>> {
    let fd = STDIN_FILENO;
    let mut termios = Termios::from_fd(fd)?;
    cfmakeraw(&mut termios); 
    tcsetattr(fd, TCSANOW, &termios)?;
    Ok(())
}


pub fn restore_terminal_settings() -> Result<(), Box<dyn Error>> {
    let fd = STDIN_FILENO;
    let mut termios = Termios::from_fd(fd)?;
    termios.c_lflag |= termios::ICANON | termios::ECHO;
    tcsetattr(fd, TCSANOW, &termios)?;
    Ok(())
}

/// Cleanly handle `Ctrl+C` (SIGINT).
/// - Restore the terminal settings.
/// - Print a message and exit with code 130.
fn handle_control_c(sig: i32) {
    let _ = restore_terminal_settings(); // Ignore errors here, just best-effort
    eprintln!("\nReceived Ctrl-C (signal = {sig}). Exiting with code 130...");
    process::exit(130);
}


pub fn spawn_control_c_handler() -> Result<(), Box<dyn Error>> {
    let signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            handle_control_c(sig);
        }
    });
    Ok(())
}

#![cfg_attr(windows, windows_subsystem = "windows")]
//! `xiv-otp` auto-enters `XIVLauncher` OTP codes in the background (windowless).
//! `xiv-otp [--setup | -s]` stores or replaces your base32 TOTP secret.
//! To run it at login, put a shortcut to xiv-otp.exe in `shell:startup`.
use std::error::Error;

mod cli;
mod console;
mod launcher;
mod secret;

use cli::Command;
use console::terminal_run;

fn main() -> Result<(), Box<dyn Error>> {
    match Command::parse(std::env::args()) {
        Ok(Command::Run) => launcher::run().or_else(|e| terminal_run(|| Err(e))),
        Ok(Command::Setup) => terminal_run(secret::setup),
        Err(usage) => terminal_run(|| Err(usage.into())),
    }
}

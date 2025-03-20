#![no_main]
#![no_std]
#![feature(used_with_arg)]

use ariel_os::debug::{ExitCode, exit, log::*};

#[ariel_os::thread(autostart)]
fn main() {
    info!("Hello World!");

    exit(ExitCode::SUCCESS);
}

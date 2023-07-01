#![windows_subsystem = "console"]
use std::error::Error;
pub mod runner;
pub mod scheduler;

// use windows::core::GUID;

// create task that run after 2mins
// based on -> https://learn.microsoft.com/en-us/windows/win32/taskschd/time-trigger-example--c---
fn main() -> Result<(), Box<dyn Error>> {
    let settings = runner::read_settings()?;
    runner::run(settings)?;
    // scheduler::win_schedule_daily(
    //     String::from("cmd.exe"),
    //     scheduler::Time {
    //         h: 18,
    //         m: 05,
    //         s: 00,
    //     },
    // )?;
    return Ok(());
}

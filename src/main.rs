use std::io::Error;
pub mod json;
pub mod scheduler;

// use windows::core::GUID;

// create task that run after 2mins
// based on -> https://learn.microsoft.com/en-us/windows/win32/taskschd/time-trigger-example--c---
fn main() -> Result<(), Error> {
    json::read()?;
    return Ok(());
}

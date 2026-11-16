use std::error::Error;

#[cfg(windows)]
unsafe extern "system" {
    unsafe fn AllocConsole() -> i32;
}

pub fn terminal_run(
    command: impl FnOnce() -> Result<(), Box<dyn Error>>,
) -> Result<(), Box<dyn Error>> {
    #[cfg(windows)]
    unsafe {
        AllocConsole()
    };

    let result = command();
    if cfg!(windows) {
        if let Err(e) = &result {
            println!("Error: {e}");
        }
        println!("Press Enter to close.");
        let _ = std::io::stdin().read_line(&mut String::new());
    }
    result
}

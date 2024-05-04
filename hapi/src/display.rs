use std::marker::PhantomData;

/// The errors for the display
#[derive(Debug)]
pub enum DisplayError {
    NotRegistered,
}

/// The display for the process
pub struct Display(PhantomData<()>);

impl Display {
    /// Push the process's stdout to the display's text buffer
    pub fn push_stdout(&mut self) -> Result<(), DisplayError> {
        let result = unsafe { crate::ffi::hapi_display_push_stdout() };
        if result == -1 {
            return Err(DisplayError::NotRegistered);
        }
        Ok(())
    }
}

/// The display server for honeyos
pub struct DisplayServer;

impl DisplayServer {
    /// Register a display
    pub fn register() -> Display {
        unsafe { crate::ffi::hapi_display_server_register() };
        Display(PhantomData)
    }

    /// Claim the display server and display as head display
    #[allow(unused_variables)]
    pub fn claim(display: &Display) -> Result<(), DisplayError> {
        let result = unsafe { crate::ffi::hapi_display_server_claim_main() };
        if result == -1 {
            return Err(DisplayError::NotRegistered);
        }
        Ok(())
    }
}

impl std::error::Error for DisplayError {}

impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayError::NotRegistered => {
                writeln!(f, "The current process does not have a display registered")
            }
        }
    }
}
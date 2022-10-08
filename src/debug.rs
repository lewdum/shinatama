#[allow(clippy::wildcard_imports)]
use windows::core::*;

#[cfg(debug_assertions)]
mod implementation {
    use windows::Win32::System::Console::{AllocConsole, FreeConsole};

    pub unsafe fn create_console() -> super::Result<()> {
        AllocConsole().ok()
    }

    pub unsafe fn destroy_console() -> super::Result<()> {
        FreeConsole().ok()
    }

    pub unsafe fn handle_error(err: &dyn std::error::Error) {
        println!("\nERROR: {}", err);
    }
}

#[cfg(not(debug_assertions))]
mod implementation {
    use std::{iter::once, process::exit};

    #[allow(clippy::wildcard_imports)]
    use windows::Win32::UI::WindowsAndMessaging::*;

    pub unsafe fn create_console() -> super::Result<()> {
        Ok(())
    }

    pub unsafe fn destroy_console() -> super::Result<()> {
        Ok(())
    }

    pub unsafe fn handle_error(err: &dyn std::error::Error) {
        let message = format!("{}.\n\nDo you want to continue anyway?", err);
        let c_str: Vec<_> = message.bytes().chain(once(0)).collect();
        if MessageBoxA(
            None,
            super::PCSTR(c_str.as_ptr()),
            super::s!("Shinatama"),
            MB_YESNO | MB_SYSTEMMODAL | MB_ICONEXCLAMATION,
        ) == IDNO
        {
            exit(1);
        }
    }
}

pub use implementation::*;

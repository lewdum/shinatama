mod config;
mod debug;
mod patch;
mod patcher;

use std::{ffi::c_void, mem};

#[allow(clippy::wildcard_imports)]
use windows::{core::*, Win32::Foundation::*, Win32::System::SystemServices::*};

use config::Config;
use patcher::Patcher;

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub extern "system" fn DllMain(_instance: HINSTANCE, reason: u32, reserved: usize) -> bool {
    match reason {
        DLL_PROCESS_ATTACH => {
            if let Err(err) = process_attach() {
                unsafe {
                    debug::handle_error(&*err);
                }
            }
        }

        DLL_PROCESS_DETACH => process_detach(reserved),

        _ => {}
    }
    true
}

fn process_attach() -> std::result::Result<(), Box<dyn std::error::Error>> {
    unsafe {
        debug::create_console()?;
    }

    println!("Loading configuration...");

    let config = Config::load()?;

    println!("Getting module handles...");

    let mut oni = unsafe { Patcher::from_main_module()? };
    let mut dao = unsafe { Patcher::from_module(s!("binkw32.dll"))? };

    println!("Applying patches...");

    unsafe {
        patch::apply_all(&config, &mut oni, &mut dao)?;
    }

    println!("Patches applied successfully.");

    Ok(())
}

fn process_detach(reserved: usize) {
    // Skip cleanup if crashing hard.
    if reserved != 0 {
        return;
    }

    unsafe {
        mem::drop(debug::destroy_console());
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn DirectInputCreateA(
    _instance: HINSTANCE,
    _version: u32,
    _direct_input: *const c_void,
    _punk_outer: *const c_void,
) -> HRESULT {
    unreachable!()
}

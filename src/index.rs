#[allow(clippy::wildcard_imports)]
use windows::{core::*, s, Win32::System::LibraryLoader::*};

pub struct Index {
    pub oni_base: usize,
    pub dao_base: usize,
}

impl Index {
    pub fn new() -> Result<Self> {
        Ok(Self {
            oni_base: get_base_address(None)?,
            dao_base: get_base_address(s!("binkw32.dll"))?,
        })
    }
}

#[allow(clippy::cast_sign_loss)]
fn get_base_address(module: impl Into<PCSTR>) -> Result<usize> {
    // Traditional optimization: handle is a pointer to the base address.
    unsafe { GetModuleHandleA(module).map(|handle| handle.0 as usize) }
}

use std::{ffi::c_void, ptr};

#[allow(clippy::wildcard_imports)]
use windows::{
    core::*,
    Win32::System::{LibraryLoader::*, Memory::*},
};

pub struct Patcher {
    base: usize,
}

impl Patcher {
    pub fn new(base: usize) -> Patcher {
        Patcher { base }
    }

    #[allow(clippy::cast_sign_loss)]
    pub unsafe fn from_main_module() -> Result<Patcher> {
        // Traditional optimization: handle is a pointer to the base address.
        GetModuleHandleA(None).map(|handle| Patcher::new(handle.0 as usize))
    }

    #[allow(clippy::cast_sign_loss)]
    pub unsafe fn from_module(module: PCSTR) -> Result<Patcher> {
        GetModuleHandleA(module).map(|handle| Patcher::new(handle.0 as usize))
    }

    // Semantically mutable.
    pub unsafe fn patch(&mut self, offset: usize, data: &[u8]) -> Result<()> {
        self.unprotected(offset, data.len(), |p| {
            ptr::copy_nonoverlapping(data.as_ptr().cast(), p, data.len());
        })
    }

    // Semantically mutable.
    pub unsafe fn patch_nop(&mut self, offset: usize, size: usize) -> Result<()> {
        self.unprotected(offset, size, |p| ptr::write_bytes(p, 0x90, size))
    }

    // Semantically mutable.
    #[inline]
    unsafe fn unprotected<F, T>(&mut self, offset: usize, size: usize, f: F) -> Result<T>
    where
        F: FnOnce(*mut c_void) -> T,
    {
        let pointer = (self.base + offset) as *mut c_void;
        let old_protection = disable_protection(pointer, size)?;
        let result = f(pointer);
        revert_protection(pointer, size, old_protection);
        Ok(result)
    }
}

unsafe fn disable_protection(pointer: *const c_void, size: usize) -> Result<PAGE_PROTECTION_FLAGS> {
    let mut old_protection = PAGE_PROTECTION_FLAGS::default();
    VirtualProtect(pointer, size, PAGE_EXECUTE_READWRITE, &mut old_protection)
        .ok()
        .map(|_| old_protection)
}

unsafe fn revert_protection(pointer: *const c_void, size: usize, mut old: PAGE_PROTECTION_FLAGS) {
    VirtualProtect(pointer, size, old, &mut old);
}

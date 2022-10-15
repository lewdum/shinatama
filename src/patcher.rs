use std::ptr;

#[allow(clippy::wildcard_imports)]
use windows::{core::*, Win32::System::Memory::*};

pub struct Patcher {
    base: usize,
}

impl Patcher {
    pub fn new(base: usize) -> Patcher {
        Patcher { base }
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
    pub unsafe fn patch_jump(&mut self, offset: usize, target: *const ()) -> Result<()> {
        let delta = (target as usize) - (self.base + offset) - 5;
        self.unprotected(offset, 5, |p| {
            ptr::write(p.cast(), 0xE9u8);
            ptr::write(p.add(1).cast(), delta);
        })
    }

    // Semantically mutable.
    pub unsafe fn patch_call(&mut self, offset: usize, target: *const ()) -> Result<()> {
        let delta = (target as usize) - (self.base + offset) - 5;
        self.unprotected(offset, 5, |p| {
            ptr::write(p.cast(), 0xE8u8);
            ptr::write(p.add(1).cast(), delta);
        })
    }

    // Semantically mutable.
    // Cannot guarantee safety because of potential race conditions.
    #[inline]
    unsafe fn unprotected<F, T>(&mut self, offset: usize, size: usize, f: F) -> Result<T>
    where
        F: FnOnce(*mut u8) -> T,
    {
        let pointer = (self.base + offset) as *mut u8;
        let old_protection = disable_protection(pointer, size)?;
        let result = f(pointer);
        revert_protection(pointer, size, old_protection);
        Ok(result)
    }
}

// Cannot guarantee safety because VirtualProtect can rarely lead to UB.
unsafe fn disable_protection<T>(pointer: *const T, size: usize) -> Result<PAGE_PROTECTION_FLAGS> {
    let mut old_protection = PAGE_PROTECTION_FLAGS::default();
    VirtualProtect(
        pointer.cast(),
        size,
        PAGE_EXECUTE_READWRITE,
        &mut old_protection,
    )
    .ok()
    .map(|_| old_protection)
}

// Cannot guarantee safety because VirtualProtect can rarely lead to UB.
unsafe fn revert_protection<T>(pointer: *const T, size: usize, mut old: PAGE_PROTECTION_FLAGS) {
    VirtualProtect(pointer.cast(), size, old, &mut old);
}

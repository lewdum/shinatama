#![allow(clippy::unreadable_literal)]

use std::mem;

#[allow(clippy::wildcard_imports)]
use windows::core::*;

use crate::{
    config::Config,
    oni::{Gun, GunSet},
    patcher::Patcher,
};

// Unsafe mainly because a bad Patcher instance could corrupt memory.
// Running on the wrong executable could also corrupt memory.
pub unsafe fn apply_all(config: &Config, oni: &mut Patcher, dao: &mut Patcher) -> Result<()> {
    if config.general.two_guns {
        oni.patch_nop(0x0F7522, 2)?;
    }
    if config.general.keep_guns {
        oni.patch_nop(0x1195FA, 4)?;
    }
    if config.general.manual_reload {
        oni.patch_nop(0x0EDC77, 5)?;
        oni.patch_nop(0x0EDC80, 5)?;
    }
    if config.general.hypo_anytime {
        oni.patch_nop(0x11C7EF, 6)?;
    }
    if config.general.no_black_bars {
        oni.patch(0x0FCEF8, b"\xDC\xE8\x90")?;
    }

    if config.development.always_dev {
        apply_always_dev(oni)?;
    }
    if config.development.unlock_doors {
        oni.patch(0x1EC5F5, b"\x01")?;
        oni.patch_nop(0x0C31EF, 6)?;
    }
    if config.development.shut_up {
        dao.patch(0x006D7C, b"\xC3")?;
    }

    if config.experimental.fix_bsl {
        apply_fix_bsl(oni)?;
    }
    if config.experimental.fast_cutscenes {
        apply_fast_cutscenes(oni)?;
    }
    if config.experimental.three_guns {
        oni.patch_call(0x0DFC49, hook_next_gun as *const _)?;
    }

    Ok(())
}

unsafe fn apply_fix_bsl(oni: &mut Patcher) -> Result<()> {
    oni.patch(0x07A5D6, b"\xE9\x76\xE8\x0A\x00")?;
    oni.patch(0x07C137, b"\xE9\x52\xCD\x0A\x00")?;
    oni.patch(0x079635, b"\xE9\x84\xF8\x0A\x00")?;
    oni.patch(
        0x128E51,
        b"\
            \x66\xFF\x88\x84\x01\x00\x00\x8B\
            \x9E\xB4\x0D\x00\x00\x8D\x5B\x42\
            \xC7\x03\x00\x00\x00\x00\x8B\xDF\
            \x8B\x98\x84\x01\x00\x00\x81\xE3\
            \xFF\xFF\x00\x00\x8A\x9C\x18\x85\
            \x01\x00\x00\x84\xDB\x74\x0B\x0F\
            \x1F\x40\x00\x66\xFF\x88\x96\x01\
            \x00\x00\xC3",
    )?;
    oni.patch(
        0x128E8E,
        b"\
            \x0F\x84\x42\x28\xF5\xFF\x53\x8B\
            \x81\xB4\x0D\x00\x00\x8B\x98\x84\
            \x01\x00\x00\x81\xE3\xFF\xFF\x00\
            \x00\x8A\x9C\x18\x85\x01\x00\x00\
            \x84\xDB\x5B\x0F\x84\x1F\x28\xF5\
            \xFF\xE9\x9B\x3C\xF5\xFF",
    )?;
    oni.patch(
        0x128EBE,
        b"\
            \x8B\x54\x24\x14\x8B\x7C\x24\x18\
            \x8B\x9F\x84\x01\x00\x00\x81\xE3\
            \xFF\xFF\x00\x00\x8A\x9C\x3B\x85\
            \x01\x00\x00\x84\xDB\x5F\x75\x09\
            \x0F\x1F\x40\x00\xE9\x56\x07\xF5\
            \xFF\xE9\x4E\x07\xF5\xFF",
    )
}

unsafe fn apply_always_dev(oni: &mut Patcher) -> Result<()> {
    oni.patch(0x0FC953, b"\xE9\x96\xC5\x02\x00")?;
    oni.patch(
        0x128EEE,
        b"\
            \x8B\x0D\x8E\xCE\x5E\x00\x85\xC9\
            \x75\x0E\x0F\x1F\x40\x00\xB9\x0B\
            \x00\x00\x00\xE8\x2A\xCD\xFC\xFF\
            \xC3",
    )
}

unsafe fn apply_fast_cutscenes(oni: &mut Patcher) -> Result<()> {
    oni.patch(0x0FBF02, b"\xE9\x19\xD0\x02\x00")?;
    oni.patch(0x0FB054, b"\xEB\x13")?;
    oni.patch(
        0x128F20,
        b"\
            \x8A\x1D\xC4\xC0\x5E\x00\x80\xFB\
            \x01\xB3\x01\x75\x0B\x0F\x1F\x40\
            \x00\x31\xC9\xEB\x05\x0F\x1F\x00\
            \x01\xC1\x89\x8E\x48\x01\x00\x00\
            \xE9\xC5\x2F\xFD\xFF",
    )
}

/// Called when swapping to a different gun with one already in hand.
/// This seems like the easiest way to do it without modifying other places.
unsafe extern "fastcall" fn hook_next_gun(
    guns: &mut GunSet,
    _zero: u32,
    out_index: &mut i16,
) -> *const Gun {
    if !guns.holstered.is_null() {
        mem::swap(&mut guns.held, &mut guns.cinematic);
    }
    *out_index = 2;
    guns.holstered
}

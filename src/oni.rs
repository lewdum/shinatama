#[repr(C)]
#[derive(Debug)]
pub struct GunSet {
    pub held: *mut Gun,
    pub cinematic: *mut Gun,
    pub holstered: *mut Gun,
}

#[repr(C)]
#[derive(Debug)]
pub struct Gun {}

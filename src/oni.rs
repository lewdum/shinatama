#[repr(C)]
#[derive(Debug)]
pub struct GunSet {
    pub held: *mut Gun,
    pub unused: *mut Gun,
    pub holstered: *mut Gun,
}

#[repr(C)]
#[derive(Debug)]
pub struct Gun {}

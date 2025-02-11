#![allow(non_camel_case_types)]

// Structures de base pour Graal
#[repr(C)]
pub struct graal_isolate_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct graal_isolatethread_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct graal_create_isolate_params_t {
    pub version: i32,
    pub reserved_address_space_size: usize,
    pub auxiliary_image_path: *const i8,
    pub auxiliary_image_reserved_space_size: usize,
    pub reserved_1: i32,
    pub reserved_2: *mut *mut i8,
    pub pkey: i32,
    pub reserved_3: i8,
    pub reserved_4: i8,
    pub reserved_5: i8,
}
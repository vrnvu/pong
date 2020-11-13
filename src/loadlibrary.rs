/*

Library only works on windows

Users of the module cannot accidentally ignore errors
Users don't need to use any ffi/C types
Users don't have to worry about C-style null-terminated strings
Users don't have to use transmute to get the procs back.
*/
use std::{ffi::c_void, mem::transmute_copy, ptr::NonNull};
use std::{ffi::CString, os::raw::c_char};

// NonNull to assert whenever we hold an instance of HModule it is not null
// Since raw pointers (*const c_void) can be nullable
// This way we know that the DLL has been successfully opened
type HModule = NonNull<c_void>;
type FarProc = NonNull<c_void>;

extern "stdcall" {
    fn LoadLibraryA(name: *const c_char) -> Option<HModule>;
}

extern "stdcall" {
    // Notice module is HModule, which is a non null type
    // Can return null if name procedure is not in the module
    fn GetProcAddress(module: HModule, name: *const c_char) -> Option<FarProc>;
}

pub struct Library {
    module: HModule,
}

impl Library {
    // Using a Rust string to hide C details
    // We could use a Result<Error, Self>
    // we assume our bin links against KERNEL32.dll
    pub fn new(name: &str) -> Option<Self> {
        /*
        C strings aren't just pointers to some bytes,
        they're pointers to a byte sequence that eventually ends with a null byte,
        otherwise it'll just keep reading and reading until it finds a null byte by accident
        */
        // Panics if name contains null sequence
        let name = CString::new(name).expect("invalid .dll name");
        let res = unsafe { LoadLibraryA(name.as_ptr()) };
        res.map(|module| Library { module })
    }

    // we take a &self because we need a valid library reference
    // option since it can also fail if we ask for an invalid name in a dll
    // Since get proc return type depends on the library its called
    // we define this method as a parametric function and return a generic
    pub fn get_proc<T>(&self, name: &str) -> Option<T> {
        let name = CString::new(name).expect("invalid proc name");
        let res = unsafe { GetProcAddress(self.module, name.as_ptr()) };
        res.map(|proc| unsafe { transmute_copy(&proc) })
    }
}

use crate::ipv4;

use crate::loadlibrary::Library;
use once_cell::sync::Lazy;
use std::ffi::c_void;

pub static FUNCTIONS: Lazy<Functions> = Lazy::new(|| {
    let lib = crate::loadlibrary::Library::new("IPHLPAPI.dll").unwrap();
    Functions {
        IcmpCreateFile: lib.get_proc("IcmpCreateFile").unwrap(),
        IcmpSendEcho: lib.get_proc("IcmpSendEcho").unwrap(),
        IcmpCloseHandle: lib.get_proc("IcmpCloseHandle").unwrap(),
    }
});
pub type Handle = *const c_void;

#[repr(C)]
#[derive(Debug)]
pub struct IpOptionInformation {
    pub ttl: u8,
    pub tos: u8,
    pub flags: u8,
    pub options_size: u8,
    pub options_data: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct IcmpEchoReply {
    pub address: ipv4::Addr,
    pub status: u32,
    pub rtt: u32,
    pub data_size: u16,
    pub reserved: u16,
    pub data: *const u8,
    pub options: IpOptionInformation,
}

type IcmpSendEcho = extern "stdcall" fn(
    handle: Handle,
    dest: ipv4::Addr,
    request_data: *const u8,
    request_size: u16,
    request_options: Option<&IpOptionInformation>,
    reply_buffer: *mut u8,
    reply_size: u32,
    timeout: u32,
) -> u32;

pub struct Functions {
    pub IcmpCreateFile: extern "stdcall" fn() -> Handle,
    pub IcmpSendEcho: extern "stdcall" fn(
        handle: Handle,
        dest: ipv4::Addr,
        request_data: *const u8,
        request_size: u16,
        request_options: Option<&IpOptionInformation>,
        reply_buffer: *mut u8,
        reply_size: u32,
        timeout: u32,
    ) -> u32,
    pub IcmpCloseHandle: extern "stdcall" fn(handle: Handle),
}

impl Functions {
    fn get() -> Self {
        let iphlp = Library::new("IPHLPAPI.dll").unwrap();
        Self {
            IcmpCreateFile: iphlp.get_proc("IcmpCreateFile").unwrap(),
            IcmpSendEcho: iphlp.get_proc("IcmpSendEcho").unwrap(),
            IcmpCloseHandle: iphlp.get_proc("IcmpCloseHandle").unwrap(),
        }
    }
}
// Temporary implementation see the memory leak
// as the resource never gets closed
#[inline(always)]
pub fn IcmpCreateFile() -> Handle {
    (FUNCTIONS.IcmpCreateFile)()
}

// Leak again
pub fn IcmpSendEcho(
    handle: Handle,
    dest: ipv4::Addr,
    request_data: *const u8,
    request_size: u16,
    request_options: Option<&IpOptionInformation>,
    reply_buffer: *mut u8,
    reply_size: u32,
    timeout: u32,
) -> u32 {
    (FUNCTIONS.IcmpSendEcho)(
        handle,
        dest,
        request_data,
        request_size,
        request_options,
        reply_buffer,
        reply_size,
        timeout,
    )
}

pub fn IcmpCloseHandle(handle: Handle) {
    (FUNCTIONS.IcmpCloseHandle)(handle)
}

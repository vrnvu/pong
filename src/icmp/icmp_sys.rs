use crate::ipv4;

use crate::loadlibrary::Library;
use std::ffi::c_void;

pub type Handle = *const c_void;

type IcmpCreateFile = extern "stdcall" fn() -> Handle;

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

// Temporary implementation see the memory leak
// as the resource never gets closed
pub fn IcmpCreateFile() -> Handle {
    let iphlp = Library::new("IPHLPAPI.dll").unwrap();
    let IcmpCreateFile: IcmpCreateFile = iphlp.get_proc("IcmpCreateFile").unwrap();
    IcmpCreateFile()
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
    let iphlp = Library::new("IPHLPAPI.dll").unwrap();
    let IcmpSendEcho: IcmpSendEcho = iphlp.get_proc("IcmpSendEcho").unwrap();
    IcmpSendEcho(
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

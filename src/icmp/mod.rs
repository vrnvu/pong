mod icmp_sys;
use crate::ipv4;

use std::mem::size_of;

pub struct Request {}
pub struct Reply {}

pub fn ping(dest: ipv4::Addr) -> Result<(), String> {
    let handle = icmp_sys::IcmpCreateFile();
    let data = "tu me dejaste de querer";

    let reply_size = size_of::<icmp_sys::IcmpEchoReply>();
    let reply_buf_size = reply_size + 8 + data.len();
    let mut reply_buf = vec![0u8; reply_buf_size];

    let timeout = 4000_u32;

    match icmp_sys::IcmpSendEcho(
        handle,
        dest,
        data.as_ptr(),
        data.len() as u16,
        None,
        reply_buf.as_mut_ptr(),
        reply_buf_size as u32,
        timeout,
    ) {
        0 => Err("IcmpSendEcho failed :(".to_string()),
        _ => Ok(()),
    }
}

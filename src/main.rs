use std::{ffi::c_void, fmt, mem::transmute, ptr::null};

type HModule = *const c_void;
type FarProc = *const c_void;

extern "stdcall" {
    fn LoadLibraryA(name: *const u8) -> HModule;
    fn GetProcAddress(module: HModule, name: *const u8) -> FarProc;
}

// type alias
type MessageBoxA = extern "stdcall" fn(*const c_void, *const u8, *const u8, u32);
struct IPAddr([u8; 4]);

#[repr(C)]
#[derive(Debug)]
struct IpOptionInformation {
    ttl: u8,
    tos: u8,
    flags: u8,
    options_size: u8,
    // actually a 32-bit pointer, but, that's a Windows
    // oddity and I couldn't find a built-in Rust type for it.
    options_data: u32,
}
impl fmt::Debug for IPAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b, c, d] = self.0;
        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}

type Handle = *const c_void;
type IcmpSendEcho = extern "stdcall" fn(
    handle: Handle,
    dest: IPAddr,
    request_data: *const u8,
    request_size: u16,
    request_options: Option<&IpOptionInformation>,
    reply_buffer: *mut u8,
    reply_size: u32,
    timeout: u32,
) -> u32;

// Request options is a pointer to an IpOptionInformation - but it could also be NULL. We could have written that field as:
//     request_options: *const IpOptionInformation,
// Both of those are FFI-safe - they are both just one regular pointer
// The former (*const X) is a lot more annoying to use from Rust code.

type IcmpCreateFile = extern "stdcall" fn() -> Handle;

#[repr(C)]
#[derive(Debug)]
struct IcmpEchoReply {
    address: IPAddr,
    status: u32,
    rtt: u32,
    data_size: u16,
    reserved: u16,
    data: *const u8,
    options: IpOptionInformation,
}

fn open_box() {
    unsafe {
        let h: HModule = LoadLibraryA("USER32.dll\0".as_ptr());
        println!("{:?}", h);

        let MessageBoxA: MessageBoxA = transmute(GetProcAddress(h, "MessageBoxA\0".as_ptr()));

        MessageBoxA(null(), "Hello from Rust\0".as_ptr(), null(), 0);
    }
}

fn print_addr_as_integer() {
    // We add the \0 to our string for C compability
    // In C strings are terminated with the \0
    let addr: IPAddr = IPAddr([8, 8, 8, 8]);
    println!("addr = {:?}", addr);

    let addr_as_integer: u32 = unsafe { transmute(addr) };
    println!("addr_as_integer = {}", addr_as_integer);
}

fn main() {
    unsafe {
        let h = LoadLibraryA("IPHLPAPI.dll\0".as_ptr());
        let IcmpCreateFile: IcmpCreateFile =
            transmute(GetProcAddress(h, "IcmpCreateFile\0".as_ptr()));
        let IcmpSendEcho: IcmpSendEcho = transmute(GetProcAddress(h, "IcmpSendEcho\0".as_ptr()));
        let handle = IcmpCreateFile();
        println!("{:?}", handle);

        let data = "tu me dejaste de querer";

        let ip_opts = IpOptionInformation {
            ttl: 128,
            tos: 0,
            flags: 0,
            options_data: 0,
            options_size: 0,
        };

        use std::mem;
        let reply_size = mem::size_of::<IcmpEchoReply>();

        let reply_buf_size = reply_size + 8 + data.len();
        let mut reply_buf = vec![0u8; reply_buf_size];

        let ret = IcmpSendEcho(
            handle,
            IPAddr([8, 8, 8, 8]),
            data.as_ptr(),
            data.len() as u16,
            Some(&ip_opts),
            reply_buf.as_mut_ptr(),
            reply_buf_size as u32,
            4000,
        );
        if ret == 0 {
            panic!("IcmpSendEcho failed! ret = {}", ret);
        }

        // casting between pointer types requires transmute:
        let reply: &IcmpEchoReply = mem::transmute(&reply_buf[0]);
        println!("{:#?}", *reply);

        // as it turns out, the "8 bytes for ICMP errors" occur *before* the
        // reply data.
        let reply_data: *const u8 = mem::transmute(&reply_buf[reply_size + 8]);
        // in the previous line, `reply_data` is just a pointer - this turns it
        // into a slice.
        let reply_data = std::slice::from_raw_parts(reply_data, reply.data_size as usize);

        use pretty_hex::*;
        println!("{:?}", reply_data.hex_dump());
    }
}

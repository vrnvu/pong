
fn main() {
    // first, let's declare both structs: with Rust repr
    struct IOI_Rust {
        ttl: u8,
        tos: u8,
        flags: u8,
        options_size: u8,
        options_data: u32,
    }

    // and C repr
    #[repr(C)]
    struct IOI_C {
        ttl: u8,
        tos: u8,
        flags: u8,
        options_size: u8,
        options_data: u32,
    }

    use memoffset::span_of;
    use std::mem::size_of;

    // let's make a quick macro, this will make this a lot easier
    macro_rules! print_offset {
        // the macro takes one identifier (the struct's name), then a tuple
        // of identifiers (the field names)
        ($type: ident, ($($field: ident),*)) => {
            // `$type` is an identifier, but we're going to
            // print it out, so we need it as a string instead.
            let t = stringify!($type);

            // this will repeat for each $field
            $(
                let f = stringify!($field);
                let span = span_of!($type, $field);
                println!("{:10} {:15} {:?}", t, f, span);
            )*

            // finally, print the total field size
            let ts = size_of::<$type>();
            println!("{:10} {:15} {}", t, "(total)", ts);
            println!();
        };
    }
    print_offset!(IOI_Rust, (ttl, tos, flags, options_size, options_data));
    print_offset!(IOI_C, (ttl, tos, flags, options_size, options_data));
}
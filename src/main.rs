mod icmp;
mod ipv4;
mod loadlibrary;

use std::{env, error::Error, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: sup DEST");
        exit(1);
    });
    icmp::ping(arg.parse()?)?;

    Ok(())
}

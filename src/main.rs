mod icmp;
mod ipv4;
mod loadlibrary;

use std::{env, error::Error, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: pong DEST");
        exit(1);
    });

    use icmp::Request;
    let dest = arg.parse()?;
    let data = "Tu me dejaste de querer";

    println!();
    println!("Pinging {:?} with {} bytes of data:", dest, data.len());

    use std::{thread::sleep, time::Duration};

    for _ in 0..4 {
        match Request::new(dest).ttl(128).timeout(4000).data(data).send() {
            Ok(res) => println!(
                "Reply from {:?}: bytes={} time={:?} TTL={}",
                res.addr,
                res.data.len(),
                res.rtt,
                res.ttl,
            ),
            Err(_) => println!("Something went wrong"),
        }

        sleep(Duration::from_secs(1));
    }
    println!();

    Ok(())
}

use std::fmt;
use std::time::SystemTime;
use log::{info, warn};

// #[macro_use]
// extern crate log;

#[derive(Debug)]
struct User {
    username: String,
    is_active_flg: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.username, self.is_active_flg)
    }
}

impl User {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    let _ = setup_logger();

    let x: u128 = "42".parse().expect("Not a number!");
    let y = {
        "432".parse::<u128>().expect("Not a number!")
    };
    println!("x => {x}; y => {y}");

    let s = String::from("hello");
    println!("s => '{s}'");

    let user1 = User {
        username: String::from("user1"),
        is_active_flg: true,
    };
    println!("user1 => '{user1}'");

    xfn();

    info!("info");
    warn!("warn");
}

fn xfn() {
    println!("Another function.");
}

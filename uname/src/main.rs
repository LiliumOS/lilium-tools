#![no_std]
#![no_main]

use alloc::{fmt::format, string::String, vec::Vec};
use ministd::{
    eprintln,
    io::{Error, ErrorKind},
    println,
};

extern crate alloc;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum PrintModes {
    KernelName,
    NodeName,
    KRelease,
    KVersion,
    Machine,
    Processor,
    HardwarePlatform,
    Os,
    __NModes,
}

const ALL_OPTS: [PrintModes; PrintModes::__NModes as usize] = [
    PrintModes::KernelName,
    PrintModes::NodeName,
    PrintModes::KRelease,
    PrintModes::KVersion,
    PrintModes::Machine,
    PrintModes::Processor,
    PrintModes::HardwarePlatform,
    PrintModes::Os,
];

fn main() -> Result<i32, Error> {
    let mut args = ministd::start::args();

    let prg_name = args.next().unwrap();

    let mut opts = Vec::with_capacity(PrintModes::__NModes as usize);

    for arg in args {
        match arg {
            "--version" => {
                println!("uname {}", std::env!("CARGO_PKG_VERSION"));
                return Ok(0);
            }
            "--help" => {
                println!("<Insert Help Here>");
                return Ok(0);
            }
            x if x.starts_with("--") => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    alloc::format!("Unknown Option {x}"),
                ));
            }
            x if x.starts_with("-") => {
                for c in (&x[1..]).chars() {
                    match c {
                        'a' => {
                            opts.extend(ALL_OPTS);
                        }
                        's' => opts.push(PrintModes::KernelName),
                        'n' => opts.push(PrintModes::NodeName),
                        'r' => opts.push(PrintModes::KRelease),
                        'v' => opts.push(PrintModes::KVersion),
                        'm' => opts.push(PrintModes::Machine),
                        'p' => opts.push(PrintModes::Processor),
                        'i' => opts.push(PrintModes::HardwarePlatform),
                        'o' => opts.push(PrintModes::Os),
                        c => {
                            return Err(Error::new(
                                ErrorKind::InvalidInput,
                                alloc::format!("Unknown Option -{c}"),
                            ));
                        }
                    }
                }
            }
        }
    }

    if opts.is_empty() {
        opts.push(PrintModes::KernelName)
    }

    let mut sys_info = Vec::with_capacity(4);

    let mut computer_name = String::new();
    let mut kernel_vendor = String::new();
    let mut os_name = String::new();

    if opts.contains(&PrintModes::KernelName)
        || opts.contains(&PrintModes::KVersion)
        || opts.contains(&PrintModes::KRelease)
    {}

    Ok(0)
}

ministd::def_main!();

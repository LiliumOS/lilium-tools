#![no_std]

use ministd::println;

pub fn help_version(name: &str, code: &str) {
    let mut args = ministd::start::args();
    let prg_name = args.next().unwrap();

    for arg in args {
        match arg {
            "--version" => {
                println!("{name} v{}", core::env!("CARGO_PKG_VERSION"));
                ministd::exit(0)
            }
            "--help" => {
                println!("Usage: {prg_name} [OPTIONS...] [--] [ARGS...]");
                println!("Trivially exits {code}");
                println!("Options:");
                println!("\t--help: Prints this message and exits");
                println!("\t--version: Prints version information and exits");
                ministd::exit(0);
            }
            x => break,
        }
    }
}

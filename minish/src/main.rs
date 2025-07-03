#![no_std]
#![no_main]
#![feature(thread_local)]

extern crate alloc;

unsafe extern "C" {
    pub safe fn exit(x: i32) -> !;
}

mod shell;

use core::panic::PanicInfo;

use alloc::string::String;
use error_repr::Error;
use lilium_sys::{
    sys::except::{ExceptionStatusInfo, UnmanagedException},
    uuid::parse_uuid,
};
use ministd::io::{self, BufReadEx, BufReader, ReadToStringError, stderr, stdin};
use ministd::{eprintln, print, println};
use shell::{parse_shell, split_shell};

use crate::shell::exec_line;

fn main() -> io::Result<i32> {
    let mut line = String::new();
    let mut reader = BufReader::new(stdin());
    loop {
        line.clear();
        print!("# ");
        let n = reader.read_line(&mut line).map_err(|e| match e {
            ReadToStringError::Read(r) => r,
            ReadToStringError::InvalidUtf8 => {
                Error::new_with_message(io::ErrorKind::InvalidData, "Invalid UTF-8 Text")
            }
        })?;
        if n == 0 {
            println!("exit");
            return Ok(0);
        }

        let line = parse_shell(split_shell(&line));

        if let Some(cmd) = &line.command {
            eprintln!("{line}");
            match exec_line(&line) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error spawning {cmd}: {e}")
                }
            }
        }
    }
}

ministd::def_main!();

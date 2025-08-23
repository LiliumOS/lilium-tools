#![no_std]
#![no_main]

use lilium_sys::sys::{
    info::{GetSystemInfo, SysInfoRequest, SysInfoRequestArchInfo, arch_info},
    kstr::KSlice,
};
use ministd::{def_main, println};

fn main() -> lilium_sys::result::Result<i32> {
    let mut args = ministd::start::args();
    let prg_name = args.next().unwrap();

    for arg in args {
        match arg {
            "--help" => {
                println!("Usage: {prg_name} [OPTION]");
                println!("Prints the host machine");
                println!("Options:");
                println!("\t--help: Prints this message and exits");
                println!("\t--version: Prints version information and exits");
                return Ok(0);
            }
            "--version" => {
                println!("arch (lilium-tools) v{}", core::env!("CARGO_PKG_VERSION"));
                return Ok(0);
            }
            x => {
                println!("{prg_name}: Unknown option {x}");
                return Ok(1);
            }
        }
    }

    let mut arch = SysInfoRequest {
        arch_info: SysInfoRequestArchInfo::INIT,
    };

    lilium_sys::result::Error::from_code(unsafe {
        GetSystemInfo(KSlice::from_slice_mut(core::slice::from_mut(&mut arch)))
    })?;

    let arch = unsafe { arch.arch_info.arch_type };

    match arch {
        arch_info::ARCH_TYPE_X86_64 => println!("x86_64"),
        arch_info::ARCH_TYPE_ARM32 => println!("arm"),
        arch_info::ARCH_TYPE_X86_IA_32 => println!("i686"),
        arch_info::ARCH_TYPE_AARCH64 => println!("aarch64"),
        arch_info::ARCH_TYPE_CLEVER_ISA => println!("clever"),
        arch_info::ARCH_TYPE_RISCV32 => println!("riscv32"),
        arch_info::ARCH_TYPE_RISCV64 => println!("riscv64"),
        arch => println!("**UNKNOWN ARCH {arch:#}**"),
    }

    Ok(0)
}

def_main!();

#![no_std]
#![no_main]

use alloc::{
    fmt::format,
    format,
    string::{String, ToString},
    vec::Vec,
};
use lilium_sys::sys::{
    error::INSUFFICIENT_LENGTH,
    info::{
        GetSystemInfo, SysInfoRequest, SysInfoRequestArchInfo, SysInfoRequestComputerName,
        SysInfoRequestKernelVendor, SysInfoRequestOsVersion,
        arch_info::{
            ARCH_TYPE_AARCH64, ARCH_TYPE_ARM32, ARCH_TYPE_CLEVER_ISA, ARCH_TYPE_RISCV32,
            ARCH_TYPE_RISCV64, ARCH_TYPE_X86_64, ARCH_TYPE_X86_IA_32,
        },
    },
    kstr::{KSlice, KStrPtr},
};
use ministd::{
    eprintln,
    helpers::get_many_mut,
    io::{Error, ErrorKind},
    print, println,
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
    eprintln!("{prg_name}");

    let mut opts = Vec::with_capacity(PrintModes::__NModes as usize);

    for arg in args {
        match arg {
            "--version" => {
                println!("uname {}", core::env!("CARGO_PKG_VERSION"));
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
                eprintln!("{x}");
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
            x => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    alloc::format!("Unknown Argument {x}"),
                ));
            }
        }
    }

    if opts.is_empty() {
        opts.push(PrintModes::KernelName)
    }

    eprintln!("{opts:?}");

    let mut sys_info = Vec::with_capacity(4);
    let mut kvendor_index = !0;
    let mut osver_index = !0;
    let mut cname_index = !0;
    let mut arch_index = !0;

    let mut computer_name = String::new();
    let mut kernel_vendor = String::new();
    let mut os_name = String::new();
    let mut sys_label = String::new();
    let mut sys_display_name = String::new();

    if opts.contains(&PrintModes::KVersion) || opts.contains(&PrintModes::KRelease) {
        kvendor_index = sys_info.len();
        sys_info.push(SysInfoRequest {
            kernel_vendor: SysInfoRequestKernelVendor {
                kvendor_name: KStrPtr {
                    str_ptr: kernel_vendor.as_mut_ptr(),
                    len: kernel_vendor.capacity(),
                },
                ..SysInfoRequestKernelVendor::INIT
            },
        });
    }

    if opts.contains(&PrintModes::Os) || opts.contains(&PrintModes::KVersion) {
        osver_index = sys_info.len();
        sys_info.push(SysInfoRequest {
            os_version: SysInfoRequestOsVersion {
                osvendor_name: KStrPtr {
                    str_ptr: os_name.as_mut_ptr(),
                    len: os_name.capacity(),
                },
                ..SysInfoRequestOsVersion::INIT
            },
        })
    }

    if opts.contains(&PrintModes::NodeName) {
        cname_index = sys_info.len();
        sys_info.push(SysInfoRequest {
            computer_name: SysInfoRequestComputerName {
                hostname: KStrPtr {
                    str_ptr: computer_name.as_mut_ptr(),
                    len: computer_name.capacity(),
                },
                sys_label: KStrPtr {
                    str_ptr: sys_label.as_mut_ptr(),
                    len: sys_label.len(),
                },
                sys_display_name: KStrPtr {
                    str_ptr: sys_display_name.as_mut_ptr(),
                    len: sys_label.len(),
                },
                ..SysInfoRequestComputerName::INIT
            },
        });
    }

    if opts.contains(&PrintModes::Machine)
        || opts.contains(&PrintModes::Processor)
        || opts.contains(&PrintModes::HardwarePlatform)
    {
        arch_index = sys_info.len();
        sys_info.push(SysInfoRequest {
            arch_info: SysInfoRequestArchInfo {
                ..SysInfoRequestArchInfo::INIT
            },
        })
    }

    loop {
        let res = unsafe { GetSystemInfo(KSlice::from_slice_mut(&mut sys_info)) };

        if res == INSUFFICIENT_LENGTH || res == 0 {
            let mut dirty = false;

            if let Some(kvendor) = sys_info.get_mut(kvendor_index) {
                let st = unsafe { &mut kvendor.kernel_vendor.kvendor_name };

                if kernel_vendor.capacity() < st.len {
                    kernel_vendor.reserve(st.len);
                    st.str_ptr = kernel_vendor.as_mut_ptr();
                    st.len = kernel_vendor.capacity();
                    dirty = true;
                } else {
                    unsafe {
                        kernel_vendor.as_mut_vec().set_len(st.len);
                    }
                }
            }

            if let Some(kvendor) = sys_info.get_mut(osver_index) {
                let st = unsafe { &mut kvendor.os_version.osvendor_name };

                if os_name.capacity() < st.len {
                    os_name.reserve(st.len);
                    st.str_ptr = os_name.as_mut_ptr();
                    st.len = os_name.capacity();
                    dirty = true;
                } else {
                    unsafe {
                        os_name.as_mut_vec().set_len(st.len);
                    }
                }
            }

            if let Some(kvendor) = sys_info.get_mut(cname_index) {
                let cname = unsafe { &mut kvendor.computer_name };

                if computer_name.capacity() < cname.hostname.len
                    || sys_label.capacity() < cname.sys_label.len
                    || sys_display_name.capacity() < cname.sys_display_name.len
                {
                    computer_name.reserve(cname.hostname.len);
                    cname.hostname.str_ptr = computer_name.as_mut_ptr();
                    cname.hostname.len = computer_name.capacity();
                    sys_label.reserve(cname.sys_label.len);
                    cname.sys_label.str_ptr = sys_label.as_mut_ptr();
                    cname.sys_label.len = sys_label.capacity();
                    sys_display_name.reserve(cname.sys_display_name.len);
                    cname.sys_display_name.str_ptr = sys_display_name.as_mut_ptr();
                    cname.sys_display_name.len = sys_display_name.capacity();
                    dirty = true;
                } else {
                    unsafe {
                        computer_name.as_mut_vec().set_len(cname.hostname.len);
                    }
                    unsafe {
                        sys_label.as_mut_vec().set_len(cname.sys_label.len);
                    }
                    unsafe {
                        sys_display_name
                            .as_mut_vec()
                            .set_len(cname.sys_display_name.len);
                    }
                }
            }

            if !dirty {
                break;
            }
        } else {
            return Err(Error::from_raw_os_error(res));
        }
    }
    let kvendor = sys_info.get(kvendor_index);
    let mach = sys_info.get(arch_index);
    let osinfo = sys_info.get(osver_index);

    let kvendor = kvendor.map(|v| unsafe { &v.kernel_vendor });
    let mach = mach.map(|v| unsafe { &v.arch_info });
    let osinfo = osinfo.map(|v| unsafe { &v.os_version });
    for info in opts {
        match info {
            PrintModes::KernelName => {
                print!("Lilium ");
            }
            PrintModes::NodeName => {
                print!("{computer_name} ");
            }
            PrintModes::KRelease => {
                let kvendor = kvendor.unwrap();
                print!(
                    "{} {}.{} ",
                    kernel_vendor, kvendor.kernel_major, kvendor.kernel_minor
                );
            }
            PrintModes::KVersion => {
                let kvendor = kvendor.unwrap();
                let osinfo = osinfo.unwrap();
                print!(
                    "{}.{} ({}.{}) ",
                    osinfo.os_major, osinfo.os_minor, kvendor.kernel_major, kvendor.kernel_minor
                );
            }
            PrintModes::Machine => {
                let mach = mach.unwrap();
                let arch_name = match mach.arch_type {
                    ARCH_TYPE_X86_64 => "x86_64",
                    ARCH_TYPE_X86_IA_32 => "i686",
                    ARCH_TYPE_AARCH64 => "aarch64",
                    ARCH_TYPE_ARM32 => "arm",
                    ARCH_TYPE_RISCV32 => "riscv32",
                    ARCH_TYPE_RISCV64 => "riscv64",
                    ARCH_TYPE_CLEVER_ISA => "Clever-ISA",
                    _ => "**UNKNOWN ARCH**!",
                };

                print!("{arch_name} ");
            }
            PrintModes::Processor => {
                let mach = mach.unwrap();
                let arch_ver = match mach.arch_type {
                    ARCH_TYPE_X86_64 if mach.arch_version > 1 => {
                        format!("x86_64v{}", mach.arch_version,)
                    }
                    ARCH_TYPE_X86_64 => "x86_64".to_string(),
                    ARCH_TYPE_X86_IA_32 => format!("i{}86", mach.arch_version),
                    ARCH_TYPE_AARCH64 => "aarch64".to_string(),
                    ARCH_TYPE_ARM32 => "arm".to_string(),
                    ARCH_TYPE_RISCV32 => "riscv32".to_string(),
                    ARCH_TYPE_RISCV64 => "riscv64".to_string(),
                    ARCH_TYPE_CLEVER_ISA => format!("Clever-ISA 1.{}", mach.arch_version),
                    id => format!("Unknown Arch {id:#}"),
                };

                print!("{arch_ver} ");
            }
            PrintModes::HardwarePlatform => {
                let mach = mach.unwrap();
                let proc_name = match mach.arch_type {
                    ARCH_TYPE_X86_64 if mach.arch_version > 1 => {
                        format!("x86_64v{}", mach.arch_version,)
                    }
                    ARCH_TYPE_X86_64 => "x86_64".to_string(),
                    ARCH_TYPE_X86_IA_32 => format!("i{}86", mach.arch_version),
                    ARCH_TYPE_AARCH64 => "aarch64".to_string(),
                    ARCH_TYPE_ARM32 => "arm".to_string(),
                    ARCH_TYPE_RISCV32 => "riscv32".to_string(),
                    ARCH_TYPE_RISCV64 => "riscv64".to_string(),
                    ARCH_TYPE_CLEVER_ISA => format!("Clever-ISA 1.{}", mach.arch_version),
                    id => format!("Unknown Arch {id:#}"),
                };

                print!("{proc_name} ");
            }
            PrintModes::Os => print!("{os_name} "),
            _ => unreachable!(),
        }
    }

    println!();

    Ok(0)
}

ministd::def_main!();

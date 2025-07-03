#![no_std]
#![feature(never_type, allocator_api, alloc_layout_extra)]

use core::panic::PanicInfo;

use lilium_sys::{
    sys::except::{ExceptionStatusInfo, UnmanagedException},
    uuid::parse_uuid,
};

use crate::io::stderr;

extern crate alloc;

pub mod helpers;
pub mod io;
pub mod start;
pub mod system;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    let _ = writeln!(stderr(), "Panicked at {}", info.message());
    unsafe {
        UnmanagedException(&ExceptionStatusInfo {
            except_code: parse_uuid("4c0c6658-59ae-5675-90c3-ffcc0a7219ad"),
            except_info: 0,
            except_reason: 0,
        })
    }
}

unsafe extern "C" {
    pub safe fn exit(x: i32) -> !;
}

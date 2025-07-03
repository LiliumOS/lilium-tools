use core::{cell::OnceCell, ffi::CStr};

use crate::{eprintln, helpers::AssertThreadSafe, println};

#[diagnostic::on_unimplemented(
    message = "Cannot return `{Self}` from `main`.",
    label = "Return type of this method is `{Self}`"
)]
pub trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}

impl Termination for ! {
    fn report(self) -> i32 {
        match self {}
    }
}

impl Termination for i32 {
    fn report(self) -> i32 {
        self
    }
}

impl<T: Termination, E: core::fmt::Debug> Termination for Result<T, E> {
    fn report(self) -> i32 {
        match self {
            Ok(val) => val.report(),
            Err(e) => {
                eprintln!(
                    "{}: {e:?}",
                    PRG_NAME
                        .get()
                        .copied()
                        .unwrap_or(c"minish")
                        .to_string_lossy()
                );
                -1
            }
        }
    }
}

// SAFETY:
// We only write this from the main thread
static PRG_NAME: AssertThreadSafe<OnceCell<&CStr>> =
    unsafe { AssertThreadSafe::new_unchecked(OnceCell::new()) };

static ARGS: AssertThreadSafe<OnceCell<(usize, *mut *mut c_char)>> =
    unsafe { AssertThreadSafe::new_unchecked(OnceCell::new()) };

static ENV: AssertThreadSafe<OnceCell<*mut *mut c_char>> =
    unsafe { AssertThreadSafe::new_unchecked(OnceCell::new()) };

#[doc(hidden)]
pub use core::ffi::c_char;

#[doc(hidden)]
pub unsafe fn rt_start<R: Termination>(
    argc: isize,
    argv: *mut *mut c_char,
    envp: *mut *mut c_char,
    main: fn() -> R,
) -> i32 {
    println!("{argc}");
    let _ = ARGS.set((argc as usize, argv));
    let _ = ENV.set(envp);
    if argc > 0 {
        let _ = PRG_NAME.set(unsafe { CStr::from_ptr(argv.read()) });
    }
    Termination::report(main())
}

#[macro_export]
macro_rules! def_main {
    () => {
        const _: () = {
            #[unsafe(export_name = "main")]
            unsafe extern "C" fn main(
                argc: isize,
                argv: *mut *mut $crate::start::c_char,
                envp: *mut *mut $crate::start::c_char,
            ) -> i32 {
                unsafe { $crate::start::rt_start(argc, argv, envp, crate::main) }
            }
        };
    };
}

pub struct Vars(*mut *mut c_char);

pub fn vars() -> Vars {
    Vars(ENV.get().copied().unwrap())
}

impl Iterator for Vars {
    type Item = (&'static str, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        let val = unsafe { self.0.read() };
        if val.is_null() {
            return None;
        }
        self.0 = unsafe { self.0.add(1) };

        let cstr = unsafe { CStr::from_ptr(val) };
        let bytes = cstr.to_bytes();

        let str = unsafe { core::str::from_utf8_unchecked(bytes) };

        let (var, val) = str.split_once('=').unwrap();

        Some((var, val))
    }
}

pub fn var(var: &str) -> Option<&'static str> {
    vars().find_map(|(key, val)| (key == var).then_some(val))
}

pub struct Args(*mut *mut c_char);

impl Iterator for Args {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let val = unsafe { self.0.read() };
        if val.is_null() {
            return None;
        }
        self.0 = unsafe { self.0.add(1) };

        let cstr = unsafe { CStr::from_ptr(val) };
        let bytes = cstr.to_bytes();

        let str = unsafe { core::str::from_utf8_unchecked(bytes) };

        Some(str)
    }
}

pub fn args() -> Args {
    Args(ARGS.get().copied().unwrap().1)
}

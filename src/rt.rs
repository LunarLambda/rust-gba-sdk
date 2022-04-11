/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#![doc(hidden)]

use core::panic::PanicInfo;

#[lang = "termination"]
pub trait Termination {
    fn report(self) -> isize;
}

impl Termination for ! {
    fn report(self) -> isize {
        self
    }
}

impl Termination for () {
    fn report(self) -> isize {
        0
    }
}

impl Termination for i32 {
    fn report(self) -> isize {
        self as isize
    }
}

#[lang = "start"]
fn lang_start<T: Termination + 'static>(user_main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize {
    user_main().report()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Needed because crt0 wants to call exit()
//
#[no_mangle]
pub extern "C" fn exit(_code: i32) -> ! {
    loop {}
}

// No idea what this does, the compiler doesn't even seem to call it but
// some ARM unwind section wants it to be there...
//
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}

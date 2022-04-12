/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::video::{Video, AnyMode};

static mut SINGLETONS: u32 = 0;

fn take(flag: u32) {
    unsafe {
        if (SINGLETONS & flag) != 0 {
            panic!("singletons have already been obtained");
        }

        SINGLETONS |= flag;
    }
}

const FLAG_VIDEO: u32 = 1 << 0;

pub fn video() -> Video<AnyMode> {
    take(FLAG_VIDEO);

    Video::new()
}

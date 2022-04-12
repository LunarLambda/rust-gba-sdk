/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#![no_std]

use gba_sdk as gba;

use gba::video::Mode3;

fn main() {
    let mut video = gba::hw::video().mode3();

    video.write(
        Mode3::new().enable_bg2()
    );

    video.write_pixel(120, 80, 0x001F);
    video.write_pixel(136, 80, 0x03E0);
    video.write_pixel(120, 96, 0x7C00);
}

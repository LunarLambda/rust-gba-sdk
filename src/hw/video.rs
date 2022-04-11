/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub struct Video {
    pub(self) dispcnt: u16,
}

pub struct Mode3 {
    inner: Video,
}

impl Video {
    pub(crate) const fn new() -> Self {
        Self {
            dispcnt: 0
        }
    }

    pub fn write(&self) {
        unsafe { (0x0400_0000 as *mut u16).write_volatile(self.dispcnt); }
    }

    pub const fn forced_blank(&self) -> bool {
        (self.dispcnt >> 7) != 0
    }

    pub fn disable_forced_blank(&mut self) {
        self.set_forced_blank(false);
    }

    pub fn enable_forced_blank(&mut self) {
        self.set_forced_blank(true);
    }

    pub fn set_forced_blank(&mut self, value: bool) {
        self.dispcnt |= (value as u16) << 7;
    }

    pub fn mode3(mut self) -> Mode3 {
        self.dispcnt &= !7;
        self.dispcnt |= 3;

        Mode3::new(self)
    }
}

impl Mode3 {
    pub(crate) const fn new(inner: Video) -> Self {
        Self {
            inner
        }
    }

    pub fn enable_bg2(&mut self) {
        self.inner.dispcnt |= 1 << 10;
    }

    pub const fn finish_mode(self) -> Video {
        self.inner
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: u16) {
        unsafe {
            (0x0600_0000 as *mut u16).offset((x + y * 240) as isize).write(c);
        }
    }
}

impl core::ops::Deref for Mode3 {
    type Target = Video;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Mode3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

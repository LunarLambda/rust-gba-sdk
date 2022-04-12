/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::marker::PhantomData;

mod sealed { pub trait Sealed {} }

pub trait Mode : sealed::Sealed + Copy + From<u16> + Into<u16> {}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnyMode(u16);

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode3(u16);

impl sealed::Sealed for AnyMode {}
impl sealed::Sealed for Mode3 {}

impl Mode for AnyMode {}
impl Mode for Mode3 {}

#[must_use = "hardware components cannot be re-acquired after dropping them"]
pub struct Video<T: Mode>(PhantomData<T>);

impl<T: Mode> Video<T> {
    pub(crate) const fn new() -> Self {
        Self(PhantomData)
    }

    pub fn write(&mut self, value: T) {
        // SAFETY: None lol
        unsafe {
            (0x0400_0000 as *mut u16).write_volatile(value.into());
        }
    }

    pub fn read(&self) -> T {
        // SAFETY: None lol
        unsafe {
            T::from((0x0400_0000 as *const u16).read_volatile())
        }
    }

    pub fn mode3(self) -> Video<Mode3> {
        Video::new()
    }
}

impl Video<Mode3> {
    pub fn write_pixel(&mut self, x: usize, y: usize, c: u16) {
        // SAFETY: None until we add bounds checking
        unsafe {
            (0x0600_0000 as *mut u16).offset((x + y * 240) as isize).write(c);
        }
    }
}

impl AnyMode {
    const DISPCNT_MASK: u16 = 0b111_10000_1_1_1_0_000;
}

impl Mode3 {
    const DISPCNT_MASK: u16 = 0b111_10100_1_1_1_0_000;
    const DISPCNT_BASE: u16 = 3;

    pub const fn new() -> Self {
        Self(Self::DISPCNT_BASE)
    }

    pub const fn enable_bg2(self) -> Self {
        Self(self.0 | (1 << 10))
    }

    pub const fn disable_bg2(self) -> Self {
        Self(self.0 & !(1 << 10))
    }

    pub const fn toggle_bg2(self) -> Self {
        Self(self.0 ^ (1 << 10))
    }

    pub const fn bg2_enabled(self, value: bool) -> Self {
        Self(self.0 | ((value as u16) << 10))
    }

    pub const fn is_bg2_enabled(self) -> bool {
        (self.0 & (1 << 10)) != 0
    }
}

impl From<u16> for Mode3 {
    fn from(value: u16) -> Self {
        Self((value & Self::DISPCNT_MASK) | Self::DISPCNT_BASE)
    }
}

impl Into<u16> for Mode3 {
    fn into(self) -> u16 {
        self.0
    }
}

impl From<u16> for AnyMode {
    fn from(value: u16) -> Self {
        Self(value & Self::DISPCNT_MASK)
    }
}

impl Into<u16> for AnyMode {
    fn into(self) -> u16 {
        self.0
    }
}

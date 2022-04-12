/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
#![no_std]
#![feature(lang_items, never_type)]

#[cfg(feature = "rt-support")]
pub mod rt;

pub mod hw;
pub mod video;

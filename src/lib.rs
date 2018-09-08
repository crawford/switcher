// Copyright 2015 Alex Crawford
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(asm, test)]
#![no_std]

//! Framework for dynamically choosing between bootable images.
//!
//! This is a very minimal framework for a first-stage bootloader for low-power ARM
//! processors. This is designed to help a system choose between multiple available
//! bootable images and then boot it.
//!
//! # Operation ##
//!
//! The general assumption is that the system will have multiple bootable images
//! (typically two) and the newer, working image of the two will be booted. In
//! order to be considered for boot, the image must have a valid checksum, it must
//! not be marked as having failed to boot, and it must have at least one available
//! boot attempt (images can be given, at most, four attempts).  When an image is
//! booted, the number of available attempts is decremented if it hasn't been
//! marked as having previously successfully booted. Once booted, images must mark
//! themselves as having successfully booted. Otherwise, the next time through the
//! boot process, the number of available attempts will be decremented, eventually
//! exhausting the available attempts and preventing that image from being used.
//!
//! # Flash Layout ##
//!
//! The layout of the flash must be compatible with the internal structures in
//! order to use this framework. The image header must be placed immediately after
//! the image it defines. This is to ease the process of checking an image's
//! checksum. The first element in the struct is the image's checksum. Since this
//! butts up against the image, the checksum can be validated by simply running the
//! image plus its checksum as one block of data through the [CRC][crc]. If the
//! image is valid, the result of the CRC will be 0.
//!
//! [crc]: https://en.wikipedia.org/wiki/Cyclic_redundancy_check
//!
//! # Examples
//!
//! ```
//! #![no_main]
//! #![no_std]
//!
//! extern crate cortex_m;
//! #[macro_use]
//! extern crate cortex_m_rt;
//! extern crate panic_halt;
//! extern crate switcher;
//!
//! use cortex_m::asm;
//! use switcher::Image;
//!
//! entry!(main);
//! fn main() -> ! {
//!     let mut image_a = unsafe { Image::from(0x1000) };
//!     let mut image_b = unsafe { Image::from(0x4000) };
//!
//!     match switcher::select(&mut image_a, &mut image_b) {
//!         Some(image) => image.boot(),
//!         None => loop {
//!             asm::wfi();
//!         },
//!     }
//! }
//! ```

#[macro_use]
extern crate bitfield;
#[cfg(test)]
extern crate test;

pub mod crc;

use core::cmp::Ordering;
use core::slice;

/// A bootable image.
///
/// Images can be created from an address using [`Image::from()`][from].
///
/// [from]: #method.from
pub struct Image<'a> {
    footer: &'a mut Footer,
}

impl<'a> Image<'a> {
    /// Creates an image from the given address.
    ///
    /// This address must point to the beginning of a valid footer (the last word in an image).
    /// This method is unsafe due to the fact that no verification is performed on the address.
    pub unsafe fn from(addr: u32) -> Image<'a> {
        Image {
            footer: (addr as *mut Footer).as_mut().unwrap(),
        }
    }

    /// Determines if the image can be booted.
    ///
    /// If the image has not been marked as having succeeded or failed to boot, its checksum will
    /// be verified and the validity recorded.
    pub fn verify_bootable(&mut self) -> bool {
        if self.footer.success() {
            return true;
        }

        if self.footer.failure() || self.footer.invalid() {
            return false;
        }

        if !self.footer.valid() {
            if crc::is_valid(unsafe {
                slice::from_raw_parts(
                    match self.footer.start_address() {
                        Some(addr) => addr as *const u8,
                        None => return false,
                    },
                    self.footer.length() as usize,
                )
            }) {
                self.footer.set_valid()
            } else {
                self.footer.set_invalid();
                return false;
            }
        }

        self.footer.attempts() > 0
    }

    /// Boots the image.
    ///
    /// If the image has not been marked as having successfully booted, the number of remaining
    /// boot attempts is decremented.
    pub fn boot(&mut self) -> ! {
        if !self.footer.success() {
            self.footer.decrement_attempts();
        }

        unsafe {
            // Set the stack pointer to 0.
            // Set the instruction pointer to the beginning of the image.
            // TODO: Move the ISV to the beginning of the image.
            asm!(
                "mov sp, $0;
                 mov ip, $1;"
                :
                :
                "r" (0),
                "r" (self.footer.start_address().unwrap())
            );
        }
        unreachable!()
    }
}

impl<'a> Ord for Image<'a> {
    fn cmp(&self, other: &Image) -> Ordering {
        self.footer.version().cmp(&other.footer.version())
    }
}

impl<'a> PartialOrd for Image<'a> {
    fn partial_cmp(&self, other: &Image) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Eq for Image<'a> {}

impl<'a> PartialEq for Image<'a> {
    fn eq(&self, other: &Image) -> bool {
        self.footer.version() == other.footer.version()
    }
}

/// Returns the newer, bootable image of the given images.
///
/// This first determines which of the images are bootable, updating the image validity if
/// necessary (see [`Image::verify_bootable()`][verify_bootable] for details). If both image are
/// bootable, the newer of the two is returned. Otherwise, the only bootable image is returned, if
/// any.
///
/// [verify_bootable]: struct.Image.html#method.verify_bootable
pub fn select<'a, 'b>(
    image_a: &'a mut Image<'b>,
    image_b: &'a mut Image<'b>,
) -> Option<&'a mut Image<'b>> {
    match (image_a.verify_bootable(), image_b.verify_bootable()) {
        (true, true) => {
            if image_a > image_b {
                Some(image_a)
            } else {
                Some(image_b)
            }
        }
        (true, false) => Some(image_a),
        (false, true) => Some(image_b),
        (false, false) => None,
    }
}

bitfield!{
    /// The footer for a bootable image.
    ///
    /// This struct should be initialized with 1s except for the length, checksum, and version when
    /// it is flashed. The struct must also follow the image it describes such that the checksum
    /// immediately follows the image.
    pub struct Footer(u32);

    /// Returns the checksum of the image.
    pub checksum, _: 23, 0;
    /// Returns the version of the image.
    pub version, _: 31, 24;
    /// Returns the length (in bytes) of the image.
    pub length, _: 55, 32;
    n_valid, set_n_valid: 56;
    n_invalid, set_n_invalid: 57;
    n_success, set_n_success: 58;
    n_failure, set_n_failure: 59;
    n_attempts, set_n_attempts: 63, 63;
}

impl Footer {
    /// Returns true if the image has been marked valid.
    pub fn valid(&self) -> bool {
        !self.n_valid()
    }
    /// Marks the image as being valid.
    pub fn set_valid(&mut self) {
        self.set_n_valid(false)
    }
    /// Returns true if the image has been marked invalid.
    pub fn invalid(&self) -> bool {
        !self.n_invalid()
    }
    /// Marks the image as being invalid.
    pub fn set_invalid(&mut self) {
        self.set_n_invalid(false)
    }
    /// Returns true if the image has been marked as having successfully booted.
    pub fn success(&self) -> bool {
        !self.n_success()
    }
    /// Returns true if the image has been marked as having failed to boot.
    pub fn failure(&self) -> bool {
        !self.n_failure()
    }
    /// Returns the number of remaining boot attempts.
    pub fn attempts(&self) -> usize {
        self.n_attempts() as usize
    }
    /// Decrements the number of remaining boot attempts by one.
    pub fn decrement_attempts(&mut self) {
        let attempts = self.n_attempts() << 1;
        self.set_n_attempts(attempts)
    }
    /// Returns the address to the start of the image.
    pub fn start_address(&self) -> Option<u32> {
        (self as *const Footer as u32).checked_sub(self.length())
    }
}

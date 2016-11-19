//! Crate for communication with PN532 (NFC chip by NXP)

#[cfg(with_i2c)]
extern crate i2cdev;

#[cfg(test)]
#[macro_use]
extern crate assert_matches;

pub mod error;
pub mod bus;
pub mod device;

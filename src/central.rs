#![no_main]
#![no_std]

use rmk::macros::rmk_central;

#[rmk_central]
mod keyboard_central {}

# Encoder 0
[[input_device.encoder]]
pin_a = "P0_29"
pin_b = "P0_02"
phase = "resolution"
resolution = 4
internal_pullup = true
reverse =  true

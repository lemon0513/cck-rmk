#![no_main]
#![no_std]

use rmk::macros::rmk_peripheral;

#[rmk_peripheral(id = 0)]
mod keyboard_peripheral {}

# Encoder 1
[[input_device.encoder]]
pin_a = "P0_29"
pin_b = "P0_02"
phase = "resolution"
resolution = 4
internal_pullup = true
reverse =  true

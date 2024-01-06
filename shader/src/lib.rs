#![no_std]

use spirv_std::glam::{vec4, Vec4};
use spirv_std::spirv;

pub mod vertex;

#[spirv(fragment)]
pub fn main_fs(output: &mut Vec4) {
    *output = vec4(1.0, 0.0, 0.0, 1.0);
}

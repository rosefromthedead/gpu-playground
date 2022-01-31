#![cfg_attr(
    target_arch = "spirv",
    feature(register_attr),
    register_attr(spirv),
    no_std
)]

use spirv_std::glam::UVec3;

extern crate spirv_std;

#[spirv(compute(threads(8)))]
pub fn main_cs(
    #[spirv(global_invocation_id)]
    gid: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)]
    buffer: &mut [u32],
) {
    buffer[gid.x as usize] = 1;
}

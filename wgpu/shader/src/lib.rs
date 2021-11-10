#![cfg_attr(
    target_arch = "spirv",
    feature(register_attr),
    register_attr(spirv),
    no_std
)]

use spirv_std::glam::UVec3;

extern crate spirv_std;

#[spirv(compute(threads(1)))]
pub fn main_cs(
    #[spirv(global_invocation_id)] gid: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] buffer: &mut [u32],
) {
    let id = ((gid.x & 0x7) << 0) | ((gid.y & 0x7) << 3) | ((gid.z & 0x7) << 6);
    buffer[id as usize] = id;
}

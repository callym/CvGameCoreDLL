#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(
  alloc_error_handler,
  alloc_layout_extra,
  allocator_api,
  core_intrinsics,
  lang_items,
  nonnull_slice_from_raw_parts,
  panic_info_message,
  slice_ptr_get
)]

extern crate alloc;

mod fassert;
mod panic;
mod system_alloc;

#[no_mangle]
pub extern "C" fn test() {
  assert_eq!(69, 420);
}

#[no_mangle]
pub extern "C" fn add(x: u32, y: u32) {
  assert_eq!(69, 420);
}

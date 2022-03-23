#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(
  alloc_error_handler,
  alloc_layout_extra,
  allocator_api,
  core_intrinsics,
  extern_types,
  lang_items,
  nonnull_slice_from_raw_parts,
  panic_info_message,
  slice_ptr_get,
  abi_thiscall,
  maybe_uninit_as_bytes
)]

extern crate alloc;

#[macro_use]
pub mod macros;

pub mod cv;

pub mod fassert;
pub mod panic;
pub mod print;
pub mod system_alloc;

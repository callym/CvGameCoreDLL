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
  abi_thiscall
)]

extern crate alloc;

#[macro_use]
mod macros;

mod cv;

mod fassert;
mod panic;
mod print;
mod system_alloc;

#[no_mangle]
pub extern "C" fn test() {
  let globals = cv::globals::Globals::new();
  let init_core = globals.init_core();

  print!("Hello, ");
  print!("World!");

  println!();

  println!("{:?}", dbg!(init_core.get_type()));

  println!("{}", init_core.get_map_script_name());
}

#[no_mangle]
pub extern "C" fn add(x: u32, y: u32) {
  assert_eq!(69, 420);
}

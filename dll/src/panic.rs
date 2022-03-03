/// When we panic/abort inside the Rust code, we can actually hook into the abort system of the DLL and show a popup.
/// The DLL shows buttons to "Ignore Once" and "Ignore Always", which actually at the moment don't work - I'm not sure
/// how safe it'd be to actually ignore a Rust panic and continue execution - probably wildly unsafe!
/// TODO: Look at how to disable these buttons for the Rust assertions.
#[panic_handler]
#[no_mangle]
pub fn panic(info: &::core::panic::PanicInfo) -> ! {
  use crate::alloc::string::String;

  let expr = info.payload().downcast_ref::<&'static str>().unwrap_or(&"");

  let mut message = String::new();
  match info.message() {
    Some(fmt) => {
      alloc::fmt::write(&mut message, *fmt).unwrap();
    },
    None => message.push_str("No error message!"),
  }

  let file = info.location().map(|loc| loc.file()).unwrap_or("");
  let line = info.location().map(|loc| loc.line()).unwrap_or(0);

  crate::fassert::f_assert_dlg(expr, &message, file, line);

  core::intrinsics::abort();
}

#[lang = "eh_personality"]
#[no_mangle]
/// cbindgen:ignore
pub extern "C" fn eh_personality() {}

// On *-pc-windows-msvc we need such a symbol to make linker happy.
#[allow(non_snake_case)]
#[no_mangle]
#[cfg(all(target_os = "windows", target_env = "msvc"))]
/// cbindgen:ignore
pub extern "C" fn __CxxFrameHandler3(
  _record: usize,
  _frame: usize,
  _context: usize,
  _dispatcher: usize,
) -> u32 {
  1
}

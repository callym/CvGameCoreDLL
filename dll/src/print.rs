use cstr_core::CString;

pub fn print(s: impl AsRef<str>) {
  let s = CString::new(s.as_ref()).unwrap();

  unsafe {
    libc::printf(s.as_ptr());
  }
}

pub fn print(s: impl AsRef<str>) {
  let s = cstr_core::CString::new(s.as_ref()).unwrap();

  unsafe {
    libc::printf(s.as_ptr());
  }
}

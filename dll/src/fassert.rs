use cstr_core::CString;

/// cbindgen:ignore
extern "C" {
  // bool FAssertDlg( const char*, const char*, const char*, unsigned int, bool& );
  fn FAssertDlg(
    szExpr: *const libc::c_char,
    szMsg: *const libc::c_char,
    szFile: *const libc::c_char,
    line: libc::c_uint,
    bIgnoreAlways: *mut bool,
  ) -> bool;
}

pub fn f_assert_dlg(
  expr: impl AsRef<str>,
  msg: impl AsRef<str>,
  file: impl AsRef<str>,
  line: u32,
) {
  let expr = CString::new(expr.as_ref()).unwrap();
  let msg = CString::new(msg.as_ref()).unwrap();
  let file = CString::new(file.as_ref()).unwrap();
  let ignore_always = &mut false;

  unsafe {
    FAssertDlg(
      expr.as_ptr(),
      msg.as_ptr(),
      file.as_ptr(),
      line,
      ignore_always as _,
    );
  }
}

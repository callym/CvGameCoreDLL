use alloc::vec::Vec;
use core::{fmt::Display, mem::MaybeUninit, ptr::NonNull};
use widestring::U16CStr;

#[repr(C)]
#[derive(Debug)]
pub struct CvWString {
  _marker: u32,
  data: [u16; 8],
  length: u32,
  capacity: u32,
}

/// cbindgen:ignore
extern "thiscall" {
  #[link_name = "??0CvWString@@QAE@PBG@Z"]
  fn CvWString_new_wchar(this: *mut CvWString, wchar: *const libc::wchar_t);
}

/// This is a view into a C++-allocated `CvWString` - we assume it lives for the length
/// of the program (`'static`) - I'm not sure how true this is!
#[derive(Debug)]
pub struct WStr {
  cpp: *const CvWString,
}

impl WStr {
  pub unsafe fn new(cpp: *const CvWString) -> Self {
    Self { cpp }
  }

  pub fn to_string(&self) -> &U16CStr {
    let cpp: &CvWString = unsafe { &*self.cpp };

    if cpp.length <= 7 {
      return U16CStr::from_slice_truncate(&cpp.data).unwrap();
    }

    let ptr: [u32; 4] = unsafe { core::mem::transmute(cpp.data) };

    unsafe { U16CStr::from_ptr_str(ptr[0] as _) }
  }
}

impl Display for WStr {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    U16CStr::display(&self.to_string()).fmt(f)
  }
}

/// This is an owned C++-allocated `CvWString`
#[derive(Debug)]
pub struct WString {
  cpp: CvWString,
}

impl WString {
  pub fn new(cpp: CvWString) -> Self {
    Self { cpp }
  }

  pub fn from_u16(bytes: Vec<u16>) -> Self {
    // The C++ fn is `new CvWString(*wchar)`, which I thought would be
    // `CvWString(*wchar) -> CvWString`
    // it actually translates to `CvWString(*alloc, *wchar)` (is this 'placement-new'?)
    // Thanks to: https://github.com/rust-lang/rust-bindgen/issues/556 for pointing me
    // in the right direction!
    // TODO: Does this need to be `ManuallyDrop` or does C++ take ownership of the bytes?
    let bytes = core::mem::ManuallyDrop::new(bytes);
    let mut cpp = MaybeUninit::zeroed();

    unsafe { CvWString_new_wchar(cpp.as_mut_ptr(), bytes.as_ptr()) };

    let cpp = unsafe { cpp.assume_init() };

    Self { cpp }
  }

  pub fn as_ptr(&self) -> NonNull<CvWString> {
    NonNull::new(&self.cpp as *const _ as _).unwrap()
  }

  pub fn to_string(&self) -> &U16CStr {
    if self.cpp.length <= 7 {
      return U16CStr::from_slice_truncate(&self.cpp.data).unwrap();
    }

    let ptr: [u32; 4] = unsafe { core::mem::transmute(self.cpp.data) };

    unsafe { U16CStr::from_ptr_str(ptr[0] as _) }
  }
}

impl Display for WString {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    U16CStr::display(&self.to_string()).fmt(f)
  }
}

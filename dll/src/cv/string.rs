use core::fmt::Display;
use widestring::U16CStr;

#[repr(C)]
#[derive(Debug)]
pub struct CvWString {
  _marker: u32,
  data: [u16; 8],
  length: u8,
  _p0: [u8; 3],
  capacity: u8,
  _p1: [u8; 3],
}

#[derive(Debug)]
pub struct WString {
  cpp: CvWString,
}

impl WString {
  pub fn new(cpp: CvWString) -> Self {
    Self { cpp }
  }

  pub fn to_string(&self) -> &U16CStr {
    if self.cpp.capacity <= 7 {
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

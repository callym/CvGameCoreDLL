use crate::cv::string::WString;
use core::ptr::NonNull;

pub mod era_info;
pub mod game_speed;

/// cbindgen:ignore
extern "thiscall" {
  pub type CvInfoBase;

  #[link_name = "?getHelp@CvInfoBase@@QBEPBGXZ"]
  fn CvInfoBase_getHelp(cvInfoBase: NonNull<CvInfoBase>) -> *const libc::wchar_t;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct InfoBase {
  cpp: NonNull<CvInfoBase>,
}

impl InfoBase {
  pub fn new(cpp: NonNull<CvInfoBase>) -> Self {
    Self { cpp }
  }

  pub fn get_help(&self) -> WString {
    unsafe { WString::from_const_wchar_t(CvInfoBase_getHelp(self.cpp)) }
  }
}

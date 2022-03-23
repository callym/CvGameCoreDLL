use super::InfoBase;
use core::ptr::NonNull;

/// cbindgen:ignore
extern "thiscall" {
  pub type CvEraInfo;

  #[link_name = "?getStartPercent@CvEraInfo@@QBEHXZ"]
  fn CvGameSpeedInfo_getStartPercent(cvGameSpeedInfo: NonNull<CvEraInfo>) -> libc::c_int;
}

#[repr(C)]
pub struct EraInfo {
  cpp: NonNull<CvEraInfo>,
}

impl EraInfo {
  pub fn new(cpp: NonNull<CvEraInfo>) -> Self {
    Self { cpp }
  }

  pub fn get_start_percent(&self) -> i32 {
    unsafe { CvGameSpeedInfo_getStartPercent(self.cpp) }
  }

  pub fn base(&self) -> InfoBase {
    InfoBase::new(unsafe { core::mem::transmute(self.cpp) })
  }
}

use core::ptr::NonNull;

use crate::cv::init_core::CvInitCore;

use super::init_core::InitCore;

/// cbindgen:ignore
extern "thiscall" {
  pub type CvGlobals;

  #[link_name = "?getInstance@CvGlobals@@SAAAV1@XZ"]
  fn CvGlobals_GetInstance() -> NonNull<CvGlobals>;

  #[link_name = "?getInitCore@CvGlobals@@QAEAAVCvInitCore@@XZ"]
  fn CvGlobals_getInitCore(cvGlobals: NonNull<CvGlobals>) -> NonNull<CvInitCore>;
}

pub struct Globals {
  cpp: NonNull<CvGlobals>,
}

impl Globals {
  pub fn new() -> Self {
    Self {
      cpp: unsafe { CvGlobals_GetInstance() },
    }
  }

  pub fn init_core(&self) -> InitCore {
    unsafe { InitCore::new(CvGlobals_getInitCore(self.cpp)) }
  }
}

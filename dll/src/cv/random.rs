use core::ptr::NonNull;

/// cbindgen:ignore
extern "thiscall" {
  pub type CvRandom;

  #[link_name = "?get@CvRandom@@QAEGGPBD@Z"]
  fn CvRandom_get(
    cvRandom: NonNull<CvRandom>,
    usNum: libc::c_ushort,
    pszLog: *const libc::c_char,
  ) -> libc::c_ushort;

  #[link_name = "?getSeed@CvRandom@@QAEKXZ"]
  fn CvRandom_getSeed(cvRandom: NonNull<CvRandom>) -> libc::c_ulong;
}

pub struct Random {
  cpp: NonNull<CvRandom>,
}

impl Random {
  pub fn new(cpp: NonNull<CvRandom>) -> Self {
    Self { cpp }
  }

  pub fn get(&self, up_to: u16) -> u16 {
    unsafe { CvRandom_get(self.cpp, up_to, core::ptr::null()) }
  }

  pub fn get_seed(&self) -> u32 {
    unsafe { CvRandom_getSeed(self.cpp) }
  }
}

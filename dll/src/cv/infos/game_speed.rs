use super::InfoBase;
use core::ptr::NonNull;

#[derive(Debug)]
#[repr(C)]
pub struct GameTurnInfo {
  pub month_increment: i32,
  pub game_turns_per_increment: i32,
}

/// cbindgen:ignore
extern "thiscall" {
  pub type CvGameSpeedInfo;

  #[link_name = "?getNumTurnIncrements@CvGameSpeedInfo@@QBEHXZ"]
  fn CvGameSpeedInfo_getNumTurnIncrements(cvGameSpeedInfo: NonNull<CvGameSpeedInfo>)
    -> libc::c_int;

  #[link_name = "?getGameTurnInfo@CvGameSpeedInfo@@QBEAAUGameTurnInfo@@H@Z"]
  fn CvGameSpeedInfo_getGameTurnInfo(
    cvGameSpeedInfo: NonNull<CvGameSpeedInfo>,
    iIndex: libc::c_int,
  ) -> *const GameTurnInfo;
}

#[repr(C)]
pub struct GameSpeedInfo {
  cpp: NonNull<CvGameSpeedInfo>,
}

impl GameSpeedInfo {
  pub fn new(cpp: NonNull<CvGameSpeedInfo>) -> Self {
    Self { cpp }
  }

  pub fn get_num_turn_increments(&self) -> i32 {
    unsafe { CvGameSpeedInfo_getNumTurnIncrements(self.cpp) }
  }

  pub fn get_game_turn_info(&self, index: i32) -> &GameTurnInfo {
    unsafe { &*CvGameSpeedInfo_getGameTurnInfo(self.cpp, index) }
  }

  pub fn base(&self) -> InfoBase {
    InfoBase::new(unsafe { core::mem::transmute(self.cpp) })
  }
}

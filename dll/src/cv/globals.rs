use crate::cv::{
  enums::{EraTypes, GameSpeed},
  infos::{
    era_info::{CvEraInfo, EraInfo},
    game_speed::{CvGameSpeedInfo, GameSpeedInfo},
  },
  init_core::{CvInitCore, InitCore},
};
use core::ptr::NonNull;

/// cbindgen:ignore
extern "thiscall" {
  pub type CvGlobals;

  #[link_name = "?getInstance@CvGlobals@@SAAAV1@XZ"]
  fn CvGlobals_GetInstance() -> NonNull<CvGlobals>;

  #[link_name = "?getInitCore@CvGlobals@@QAEAAVCvInitCore@@XZ"]
  fn CvGlobals_getInitCore(cvGlobals: NonNull<CvGlobals>) -> NonNull<CvInitCore>;

  #[link_name = "?getLoadedInitCore@CvGlobals@@QAEAAVCvInitCore@@XZ"]
  fn CvGlobals_getLoadedInitCore(cvGlobals: NonNull<CvGlobals>) -> NonNull<CvInitCore>;

  #[link_name = "?getIniInitCore@CvGlobals@@QAEAAVCvInitCore@@XZ"]
  fn CvGlobals_getIniInitCore(cvGlobals: NonNull<CvGlobals>) -> NonNull<CvInitCore>;

  #[link_name = "?getMaxCivPlayers@CvGlobals@@QBEHXZ"]
  fn CvGlobals_getMaxCivPlayers(cvGlobals: NonNull<CvGlobals>) -> libc::c_int;

  #[link_name = "?getGameSpeedInfo@CvGlobals@@QAEAAVCvGameSpeedInfo@@W4GameSpeedTypes@@@Z"]
  fn CvGlobals_getGameSpeedInfo(
    cvGlobals: NonNull<CvGlobals>,
    eEraNum: libc::c_int,
  ) -> NonNull<CvGameSpeedInfo>;

  #[link_name = "?getEraInfo@CvGlobals@@QAEAAVCvEraInfo@@W4EraTypes@@@Z"]
  fn CvGlobals_getEraInfo(
    cvGlobals: NonNull<CvGlobals>,
    eGameSpeedNum: libc::c_int,
  ) -> NonNull<CvEraInfo>;
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

  pub fn loaded_init_core(&self) -> InitCore {
    unsafe { InitCore::new(CvGlobals_getLoadedInitCore(self.cpp)) }
  }

  pub fn ini_init_core(&self) -> InitCore {
    unsafe { InitCore::new(CvGlobals_getIniInitCore(self.cpp)) }
  }

  pub fn max_players(&self) -> i32 {
    unsafe { CvGlobals_getMaxCivPlayers(self.cpp) }
  }

  pub fn get_game_speed_info(&self, ty: GameSpeed) -> GameSpeedInfo {
    unsafe { GameSpeedInfo::new(CvGlobals_getGameSpeedInfo(self.cpp, ty.into())) }
  }

  pub fn get_era_info(&self, ty: EraTypes) -> EraInfo {
    unsafe { EraInfo::new(CvGlobals_getEraInfo(self.cpp, ty.into())) }
  }
}

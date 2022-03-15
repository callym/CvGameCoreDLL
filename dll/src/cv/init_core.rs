use super::string::{CvWString, WString};
use core::ptr::NonNull;

/// cbindgen:ignore
extern "thiscall" {
  pub type CvInitCore;

  #[link_name = "?getType@CvInitCore@@QBE?AW4GameType@@XZ"]
  fn CvInitCore_getType(cvInitCore: NonNull<CvInitCore>) -> i32;

  #[link_name = "?getMapScriptName@CvInitCore@@QBE?AVCvWString@@XZ"]
  fn CvInitCore_getMapScriptName(cvInitCore: NonNull<CvInitCore>) -> CvWString;
}

#[derive(Debug, num_enum::TryFromPrimitive)]
#[repr(i32)]
pub enum GameType {
  GameNone = -1,

  GameSpNew,
  GameSpScenario,
  GameSpLoad,
  GameMpNew,
  GameMpScenario,
  GameMpLoad,
  GameHotseatNew,
  GameHotseatScenario,
  GameHotseatLoad,
  GamePbemNew,
  GamePbemScenario,
  GamePbemLoad,
  GameReplay,
  NumGametypes,
}

pub struct InitCore {
  cpp: NonNull<CvInitCore>,
}

impl InitCore {
  pub fn new(cpp: NonNull<CvInitCore>) -> Self {
    Self { cpp }
  }

  pub fn get_type(&self) -> GameType {
    unsafe { CvInitCore_getType(self.cpp).try_into().unwrap() }
  }

  pub fn get_map_script_name(&self) -> WString {
    unsafe { WString::new(CvInitCore_getMapScriptName(self.cpp)) }
  }
}

use crate::cv::{
  enums::{
    EraTypes,
    GameMode,
    GameOptionTypes,
    GameSpeed,
    GameType,
    MultiplayerOptionTypes,
    PlayerTypes,
    SlotStatus,
    TeamTypes,
  },
  string::{CvWString, WStr, WString},
};
use alloc::vec::Vec;
use core::ptr::NonNull;

/// cbindgen:ignore
extern "thiscall" {
  pub type CvInitCore;

  #[link_name = "?setAdminPassword@CvInitCore@@QAEXABVCvWString@@_N@Z"]
  fn CvInitCore_setAdminPassword(
    cvInitCore: NonNull<CvInitCore>,
    szAdminPassword: NonNull<CvWString>,
    _: bool,
  );

  #[link_name = "?getType@CvInitCore@@QBE?AW4GameType@@XZ"]
  fn CvInitCore_getType(cvInitCore: NonNull<CvInitCore>) -> libc::c_int;

  #[link_name = "?getMapScriptName@CvInitCore@@QBE?AVCvWString@@XZ"]
  fn CvInitCore_getMapScriptName(cvInitCore: NonNull<CvInitCore>) -> CvWString;

  #[link_name = "?getCivAdjectiveKey@CvInitCore@@QBEABVCvWString@@W4PlayerTypes@@@Z"]
  fn CvInitCore_getCivAdjectiveKey(
    cvInitCore: NonNull<CvInitCore>,
    eID: libc::c_int,
  ) -> *const CvWString;

  #[link_name = "?getMode@CvInitCore@@QBE?AW4GameMode@@XZ"]
  fn CvInitCore_getMode(cvInitCore: NonNull<CvInitCore>) -> libc::c_int;

  #[link_name = "?getMPOption@CvInitCore@@QBE_NW4MultiplayerOptionTypes@@@Z"]
  fn CvInitCore_getMPOption(
    cvInitCore: NonNull<CvInitCore>,
    eIndex: MultiplayerOptionTypes,
  ) -> bool;

  #[link_name = "?setMPOption@CvInitCore@@QAEXW4MultiplayerOptionTypes@@_N@Z"]
  fn CvInitCore_setMPOption(
    cvInitCore: NonNull<CvInitCore>,
    eIndex: MultiplayerOptionTypes,
    bbMPOption: bool,
  );

  #[link_name = "?getOption@CvInitCore@@QBE_NW4GameOptionTypes@@@Z"]
  fn CvInitCore_getOption(cvInitCore: NonNull<CvInitCore>, eIndex: GameOptionTypes) -> bool;

  #[link_name = "?setOption@CvInitCore@@QAEXW4GameOptionTypes@@_N@Z"]
  fn CvInitCore_setOption(
    cvInitCore: NonNull<CvInitCore>,
    eIndex: GameOptionTypes,
    bbMPOption: bool,
  );

  #[link_name = "?getSlotStatus@CvInitCore@@QBE?AW4SlotStatus@@W4PlayerTypes@@@Z"]
  fn CvInitCore_getSlotStatus(cvInitCore: NonNull<CvInitCore>, eID: libc::c_int) -> libc::c_int;

  #[link_name = "?getPitbossTurnTime@CvInitCore@@QBEHXZ"]
  fn CvInitCore_getPitbossTurnTime(cvInitCore: NonNull<CvInitCore>) -> libc::c_int;

  #[link_name = "?getTeam@CvInitCore@@QBE?AW4TeamTypes@@W4PlayerTypes@@@Z"]
  fn CvInitCore_getTeam(cvInitCore: NonNull<CvInitCore>, eID: libc::c_int) -> libc::c_int;

  #[link_name = "?setTeam@CvInitCore@@QAEXW4PlayerTypes@@W4TeamTypes@@@Z"]
  fn CvInitCore_setTeam(cvInitCore: NonNull<CvInitCore>, eID: libc::c_int, eTeam: libc::c_int);

  #[link_name = "?getGameTurn@CvInitCore@@QBEHXZ"]
  fn CvInitCore_getGameTurn(cvInitCore: NonNull<CvInitCore>) -> libc::c_int;

  #[link_name = "?getGameSpeed@CvInitCore@@QBE?AW4GameSpeedTypes@@XZ"]
  fn CvInitCore_getGameSpeed(cvInitCore: NonNull<CvInitCore>) -> libc::c_int;

  #[link_name = "?getEra@CvInitCore@@QBE?AW4EraTypes@@XZ"]
  fn CvInitCore_getEra(cvInitCore: NonNull<CvInitCore>) -> libc::c_int;
}

pub struct InitCore {
  cpp: NonNull<CvInitCore>,
}

impl InitCore {
  pub fn new(cpp: NonNull<CvInitCore>) -> Self {
    Self { cpp }
  }

  pub fn set_admin_password(&self, password: Vec<u16>) {
    let password = WString::from_u16(password);
    unsafe { CvInitCore_setAdminPassword(self.cpp, password.as_ptr(), true) }
  }

  pub fn get_type(&self) -> GameType {
    unsafe { CvInitCore_getType(self.cpp).try_into().unwrap() }
  }

  pub fn get_mode(&self) -> GameMode {
    unsafe { CvInitCore_getMode(self.cpp).try_into().unwrap() }
  }

  pub fn get_map_script_name(&self) -> WString {
    unsafe { WString::new(CvInitCore_getMapScriptName(self.cpp)) }
  }

  pub fn get_civ_adjective_key(&self, slot: PlayerTypes) -> WStr {
    unsafe { WStr::new(CvInitCore_getCivAdjectiveKey(self.cpp, slot.into())) }
  }

  pub fn get_mp_option(&self, option: MultiplayerOptionTypes) -> bool {
    unsafe { CvInitCore_getMPOption(self.cpp, option) }
  }

  pub fn set_mp_option(&self, option: MultiplayerOptionTypes, set: bool) {
    unsafe { CvInitCore_setMPOption(self.cpp, option, set) };
  }

  pub fn get_option(&self, option: GameOptionTypes) -> bool {
    unsafe { CvInitCore_getOption(self.cpp, option) }
  }

  pub fn set_option(&self, option: GameOptionTypes, set: bool) {
    unsafe { CvInitCore_setOption(self.cpp, option, set) };
  }

  pub fn get_pitboss_turn_time(&self) -> i32 {
    unsafe { CvInitCore_getPitbossTurnTime(self.cpp) }
  }

  pub fn get_slot_status(&self, slot: PlayerTypes) -> SlotStatus {
    unsafe {
      CvInitCore_getSlotStatus(self.cpp, slot.into())
        .try_into()
        .unwrap()
    }
  }

  pub fn get_team(&self, slot: PlayerTypes) -> TeamTypes {
    unsafe {
      CvInitCore_getTeam(self.cpp, slot.into())
        .try_into()
        .unwrap()
    }
  }

  pub fn set_team(&self, slot: PlayerTypes, team: TeamTypes) {
    unsafe {
      CvInitCore_setTeam(self.cpp, slot.into(), team.into());
    }
  }

  pub fn get_game_turn(&self) -> i32 {
    unsafe { CvInitCore_getGameTurn(self.cpp) }
  }

  pub fn get_game_speed(&self) -> GameSpeed {
    unsafe { CvInitCore_getGameSpeed(self.cpp).try_into().unwrap() }
  }

  pub fn get_era(&self) -> EraTypes {
    unsafe { CvInitCore_getEra(self.cpp).try_into().unwrap() }
  }
}

#[derive(Debug, num_enum::TryFromPrimitive, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameType {
  None = -1,
  SPNew,
  SPScenario,
  SPLoad,
  MPNew,
  MPScenario,
  MPLoad,
  HotseatNew,
  HotseatScenario,
  HotseatLoad,
  PBEMNew,
  PBEMScenario,
  PBEMLoad,
  Replay,
  NumTypes,
}

impl GameType {
  pub fn is_single_player(self) -> bool {
    self == GameType::SPNew || self == GameType::SPScenario
  }

  pub fn is_network_multiplayer(self) -> bool {
    self == GameType::MPNew || self == GameType::MPScenario || self == GameType::MPScenario
  }

  pub fn is_hot_seat(self) -> bool {
    self == GameType::HotseatNew
      || self == GameType::HotseatScenario
      || self == GameType::HotseatLoad
  }

  pub fn is_pbem(self) -> bool {
    self == GameType::PBEMNew || self == GameType::PBEMScenario || self == GameType::PBEMLoad
  }

  pub fn is_multiplayer(self) -> bool {
    self.is_network_multiplayer() || self.is_pbem() || self.is_hot_seat()
  }
}

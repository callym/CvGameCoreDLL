#[derive(Debug, num_enum::TryFromPrimitive, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameMode {
  No = -1,
  Normal,
  Pitboss,
  NumModes,
}

#[derive(Debug, num_enum::TryFromPrimitive, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MultiplayerOptionTypes {
  No = -1,
  SimultaneousTurns,
  TakeoverAi,
  ShuffleTeams,
  Anonymous,
  TurnTimer,
  NumTypes,
}

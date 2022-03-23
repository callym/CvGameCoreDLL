#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerTypes {
  No,
  Yes(i32),
}

impl From<i32> for PlayerTypes {
  fn from(i: i32) -> Self {
    if i == -1 {
      PlayerTypes::No
    } else {
      PlayerTypes::Yes(i)
    }
  }
}

impl Into<i32> for PlayerTypes {
  fn into(self) -> i32 {
    match self {
      PlayerTypes::No => -1,
      PlayerTypes::Yes(i) => i,
    }
  }
}

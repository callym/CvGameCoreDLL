#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameSpeed {
  No,
  Yes(i32),
}

impl From<i32> for GameSpeed {
  fn from(i: i32) -> Self {
    if i == -1 {
      GameSpeed::No
    } else {
      GameSpeed::Yes(i)
    }
  }
}

impl Into<i32> for GameSpeed {
  fn into(self) -> i32 {
    match self {
      GameSpeed::No => -1,
      GameSpeed::Yes(i) => i,
    }
  }
}

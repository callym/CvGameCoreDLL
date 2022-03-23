#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EraTypes {
  No,
  Yes(i32),
}

impl From<i32> for EraTypes {
  fn from(i: i32) -> Self {
    if i == -1 {
      EraTypes::No
    } else {
      EraTypes::Yes(i)
    }
  }
}

impl Into<i32> for EraTypes {
  fn into(self) -> i32 {
    match self {
      EraTypes::No => -1,
      EraTypes::Yes(i) => i,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeamTypes {
  No,
  Yes(i32),
}

impl From<i32> for TeamTypes {
  fn from(i: i32) -> Self {
    if i == -1 {
      TeamTypes::No
    } else {
      TeamTypes::Yes(i)
    }
  }
}

impl Into<i32> for TeamTypes {
  fn into(self) -> i32 {
    match self {
      TeamTypes::No => -1,
      TeamTypes::Yes(i) => i,
    }
  }
}

macro_rules! yes_no_enum {
  ($($name:ident),* $(,)?) => {
    $(#[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum $name {
      No,
      Yes(i32),
    }

    impl From<i32> for $name {
      fn from(i: i32) -> Self {
        if i == -1 {
          Self::No
        } else {
          Self::Yes(i)
        }
      }
    }

    impl Into<i32> for $name {
      fn into(self) -> i32 {
        match self {
          Self::No => -1,
          Self::Yes(i) => i,
        }
      }
    })*
  };
}

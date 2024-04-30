use {crate::CellValue, std::fmt::Display};

impl CellValue {
  #[must_use]
  pub const fn to_byte(&self) -> u8 {
    match self {
      Self::Empty => 0,
      Self::X => 1,
      Self::O => 2,
    }
  }

  #[must_use]
  pub const fn from_byte(byte: u8) -> Self {
    match byte {
      0 => Self::Empty,
      1 => Self::X,
      2 => Self::O,
      _ => unreachable!(),
    }
  }
}

impl Display for CellValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Empty => " ",
        Self::X => "X",
        Self::O => "O",
      }
    )
  }
}

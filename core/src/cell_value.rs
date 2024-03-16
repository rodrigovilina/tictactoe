use crate::CellValue;

impl CellValue {
  #[must_use] pub fn to_byte(&self) -> u8 {
    match self {
      Self::Empty => 0,
      Self::X => 1,
      Self::O => 2,
    }
  }

  #[must_use] pub fn from_byte(byte: u8) -> Self {
    match byte {
      0 => Self::Empty,
      1 => Self::X,
      2 => Self::O,
      _ => unreachable!(),
    }
  }
}

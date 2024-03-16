use crate::{CellValue, Player};

impl Player {
  #[must_use] pub fn other(player: Self) -> Self {
    match player {
      Self::X => Self::O,
      Self::O => Self::X,
    }
  }

  #[must_use] pub fn to_cell_value(&self) -> CellValue {
    match self {
      Self::X => CellValue::X,
      Self::O => CellValue::O,
    }
  }

  #[must_use] pub fn to_byte(&self) -> u8 {
    match self {
      Self::X => 0,
      Self::O => 1,
    }
  }

  #[must_use] pub fn from_byte(byte: u8) -> Self {
    match byte {
      0 => Self::X,
      1 => Self::O,
      _ => unreachable!(),
    }
  }
}

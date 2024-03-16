use crate::CellValue;

impl CellValue {
  pub fn to_byte(&self) -> u8 {
    match self {
      CellValue::Empty => 0,
      CellValue::X => 1,
      CellValue::O => 2,
    }
  }

  pub fn from_byte(byte: u8) -> Self {
    match byte {
      0 => CellValue::Empty,
      1 => CellValue::X,
      2 => CellValue::O,
      _ => unreachable!(),
    }
  }
}

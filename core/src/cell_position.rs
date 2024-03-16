use crate::CellPosition;

impl CellPosition {
  pub fn to_byte(&self) -> u8 {
    match self {
      CellPosition::TopLeft => 0,
      CellPosition::Top => 1,
      CellPosition::TopRight => 2,
      CellPosition::Left => 3,
      CellPosition::Center => 4,
      CellPosition::Right => 5,
      CellPosition::BottomLeft => 6,
      CellPosition::Bottom => 7,
      CellPosition::BottomRight => 8,
    }
  }

  pub fn from_byte(byte: u8) -> Self {
    match byte {
      0 => CellPosition::TopLeft,
      1 => CellPosition::Top,
      2 => CellPosition::TopRight,
      3 => CellPosition::Left,
      4 => CellPosition::Center,
      5 => CellPosition::Right,
      6 => CellPosition::BottomLeft,
      7 => CellPosition::Bottom,
      8 => CellPosition::BottomRight,
      _ => unreachable!(),
    }
  }
}

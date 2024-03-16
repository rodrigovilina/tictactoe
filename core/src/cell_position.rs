use crate::CellPosition;

impl CellPosition {
  #[must_use] pub fn to_byte(&self) -> u8 {
    match self {
      Self::TopLeft => 0,
      Self::Top => 1,
      Self::TopRight => 2,
      Self::Left => 3,
      Self::Center => 4,
      Self::Right => 5,
      Self::BottomLeft => 6,
      Self::Bottom => 7,
      Self::BottomRight => 8,
    }
  }

  #[must_use] pub fn from_byte(byte: u8) -> Self {
    match byte {
      0 => Self::TopLeft,
      1 => Self::Top,
      2 => Self::TopRight,
      3 => Self::Left,
      4 => Self::Center,
      5 => Self::Right,
      6 => Self::BottomLeft,
      7 => Self::Bottom,
      8 => Self::BottomRight,
      _ => unreachable!(),
    }
  }
}

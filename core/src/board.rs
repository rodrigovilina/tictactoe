use crate::{Board, CellPosition, CellValue};

impl Board {
  pub fn is_cell_empty(&self, position: CellPosition) -> bool {
    match position {
      CellPosition::TopLeft => self.top_left == CellValue::Empty,
      CellPosition::Top => self.top == CellValue::Empty,
      CellPosition::TopRight => self.top_right == CellValue::Empty,
      CellPosition::Left => self.left == CellValue::Empty,
      CellPosition::Center => self.center == CellValue::Empty,
      CellPosition::Right => self.right == CellValue::Empty,
      CellPosition::BottomLeft => self.bottom_left == CellValue::Empty,
      CellPosition::Bottom => self.bottom == CellValue::Empty,
      CellPosition::BottomRight => self.bottom_right == CellValue::Empty,
    }
  }

  pub fn play_cell(&mut self, value: CellValue, position: CellPosition) {
    match position {
      CellPosition::TopLeft => self.top_left = value,
      CellPosition::Top => self.top = value,
      CellPosition::TopRight => self.top_right = value,
      CellPosition::Left => self.left = value,
      CellPosition::Center => self.center = value,
      CellPosition::Right => self.right = value,
      CellPosition::BottomLeft => self.bottom_left = value,
      CellPosition::Bottom => self.bottom = value,
      CellPosition::BottomRight => self.bottom_right = value,
    }
  }
}

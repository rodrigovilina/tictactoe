use crate::{Board, CellPosition, CellValue, Player};

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

  pub fn play_cell(&mut self, player: Player, position: CellPosition) {
    match position {
      CellPosition::TopLeft => self.top_left = player.to_cell_value(),
      CellPosition::Top => self.top = player.to_cell_value(),
      CellPosition::TopRight => self.top_right = player.to_cell_value(),
      CellPosition::Left => self.left = player.to_cell_value(),
      CellPosition::Center => self.center = player.to_cell_value(),
      CellPosition::Right => self.right = player.to_cell_value(),
      CellPosition::BottomLeft => self.bottom_left = player.to_cell_value(),
      CellPosition::Bottom => self.bottom = player.to_cell_value(),
      CellPosition::BottomRight => self.bottom_right = player.to_cell_value(),
    }
  }
}

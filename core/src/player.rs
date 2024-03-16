use crate::{CellValue, Player};

impl Player {
  pub fn other(player: Player) -> Player {
    match player {
      Player::X => Player::O,
      Player::O => Player::X,
    }
  }

  pub fn to_cell_value(&self) -> CellValue {
    match self {
      Player::X => CellValue::X,
      Player::O => CellValue::O,
    }
  }

  pub fn to_byte(&self) -> u8 {
    match self {
      Player::X => 0,
      Player::O => 1,
    }
  }

  pub fn from_byte(byte: u8) -> Self {
    match byte {
      0 => Player::X,
      1 => Player::O,
      _ => unreachable!(),
    }
  }
}

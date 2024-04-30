use crate::{CellPosition, CellValue, Packet, Player};

impl Packet {
  pub fn to_bytes(self) -> Vec<u8> {
    match self {
      Self::AttemptPlay { player, position } => {
        vec![0, player.to_byte(), position.to_byte()]
      }
      Self::Play { value, position } => {
        vec![1, value.to_byte(), position.to_byte()]
      }
      Self::AssignPlayer { player } => {
        vec![2, player.to_byte()]
      }
    }
  }

  pub fn from_bytes(bytes: &[u8]) -> Self {
    match bytes {
      [0, a, b] => {
        let player: Player = Player::from_byte(*a);
        let position: CellPosition = CellPosition::from_byte(*b);
        Self::AttemptPlay { player, position }
      }
      [1, a, b] => {
        let value: CellValue = CellValue::from_byte(*a);
        let position: CellPosition = CellPosition::from_byte(*b);
        Self::Play { value, position }
      }
      [2, a] => {
        let player: Player = Player::from_byte(*a);
        Self::AssignPlayer { player }
      }
      _ => unreachable!(),
    }
  }
}

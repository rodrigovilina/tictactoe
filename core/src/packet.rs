use crate::{CellPosition, CellValue, Packet, Player};

impl Packet {
  pub fn to_bytes(&self) -> Vec<u8> {
    match self {
      Self::AttemptPlay { player, position } => {
        vec![0, player.to_byte(), position.to_byte()]
      }
      Self::Play { value, position } => {
        vec![1, value.to_byte(), position.to_byte()]
      }
    }
  }

  pub fn from_bytes(bytes: Vec<u8>) -> Self {
    match bytes.first().unwrap() {
      0 => {
        let player: Player = Player::from_byte(*bytes.get(1).unwrap());
        let position: CellPosition = CellPosition::from_byte(*bytes.get(2).unwrap());
        Self::AttemptPlay { player, position }
      }
      1 => {
        let value: CellValue = CellValue::from_byte(*bytes.get(1).unwrap());
        let position: CellPosition = CellPosition::from_byte(*bytes.get(2).unwrap());
        Self::Play { value, position }
      }
      _ => unreachable!(),
    }
  }
}

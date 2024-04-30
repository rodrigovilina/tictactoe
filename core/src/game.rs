use crate::{AttemptPlay, CellPosition, CellValue, Game, Player};

impl AttemptPlay for Game {
  fn attempt_play(&mut self, player: Player, position: CellPosition) -> bool {
    if self.current_turn == player {
      if self.board.is_cell_empty(position) {
        self.board.play_cell(player.to_cell_value(), position);
      } else { return false }
      self.current_turn = Player::other(player);
      return true
    } 
    false
  }
}

impl Game {
  pub fn play(&mut self, value: CellValue, position: CellPosition) {
    self.board.play_cell(value, position);
    self.current_turn = Player::other(self.current_turn);
  }

  pub fn draw(&self) {
    println!("{}|{}|{}", self.board.top_left, self.board.top, self.board.top_right);
    println!("-+-+-");
    println!("{}|{}|{}", self.board.left, self.board.center, self.board.right);
    println!("-+-+-");
    println!("{}|{}|{}", self.board.bottom_left, self.board.bottom, self.board.bottom_right);
  }
}

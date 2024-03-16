use crate::{AttemptPlay, CellPosition, Game, Player};

impl AttemptPlay for Game {
  fn attempt_play(&mut self, player: Player, position: CellPosition) {
    if self.current_turn == player {
      if self.board.is_cell_empty(position) {
        self.board.play_cell(player, position)
      }
      self.current_turn = Player::other(player)
    }
  }
}

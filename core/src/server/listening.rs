use {
  crate::{ClientHandler, Connected, Listening, Packet, Player, Server},
  std::{io::Write, net::TcpStream},
};

impl Server<Listening> {
  #[must_use] pub fn connect(self) -> Server<Connected> {
    println!("waiting for player 1");
    let mut player_x: TcpStream = self.state.tcp_listener.accept().unwrap().0;
    let packet = Packet::AssignPlayer { player: Player::X };
    match player_x.write(&packet.to_bytes()) {
      Ok(2) => println!("Player 1 assigned Player::X"),
      Ok(_) | Err(_) => panic!()
    }
    println!("player 1 connected");
    println!("waiting for player 2");
    let mut player_o: TcpStream = self.state.tcp_listener.accept().unwrap().0;
    let packet = Packet::AssignPlayer { player: Player::O };
    match player_o.write(&packet.to_bytes()) {
      Ok(2) => println!("Player 2 assigned Player::O"),
      Ok(_) | Err(_) => panic!()
    }
    println!("player 2 connected");

    Server::<Connected> {
      game: self.game,
      state: Connected {
        player_x: ClientHandler { stream: player_x },
        player_o: ClientHandler { stream: player_o },
      },
    }
  }
}

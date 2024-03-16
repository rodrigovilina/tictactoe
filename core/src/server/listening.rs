use {
  crate::{ClientHandler, Connected, Listening, Server},
  std::net::TcpStream,
};

impl Server<Listening> {
  #[must_use] pub fn connect(self) -> Server<Connected> {
    println!("waiting for player 1");
    let player_x: TcpStream = self.state.tcp_listener.accept().unwrap().0;
    println!("player 1 connected");
    println!("waiting for player 2");
    let player_o: TcpStream = self.state.tcp_listener.accept().unwrap().0;
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

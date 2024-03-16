use {
  crate::{Client, ClientConnected, Game, Init, ServerHandler},
  std::net::TcpStream,
};

impl Client<Init> {
  pub fn new() -> Self {
    Self {
      game: Game::default(),
      state: Init(),
    }
  }

  pub fn connect(self) -> Client<ClientConnected> {
    let stream: TcpStream = TcpStream::connect("127.0.0.1:8754").unwrap();
    Client::<ClientConnected> {
      game: self.game,
      state: ClientConnected {
        server: ServerHandler { stream },
      },
    }
  }
}

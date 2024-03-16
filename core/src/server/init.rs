use {
  crate::{Game, Init, Listening, Server},
  std::net::TcpListener,
};

impl Default for Server<Init> {
    fn default() -> Self {
        Self::new()
    }
}

impl Server<Init> {
  #[must_use] pub fn new() -> Self {
    Self {
      game: Game::default(),
      state: Init(),
    }
  }

  #[must_use] pub fn listen(self) -> Server<Listening> {
    let tcp_listener: TcpListener = TcpListener::bind("127.0.0.1:8754").unwrap();

    Server::<Listening> {
      game: self.game,
      state: Listening { tcp_listener },
    }
  }
}

use {
  crate::{Client, ClientConnected, Game, Init, Packet, Player, ServerHandler},
  std::{error::Error, io::Read, net::TcpStream},
};

impl Default for Client<Init> {
  fn default() -> Self {
    Self::new()
  }
}

impl Client<Init> {
  #[must_use]
  pub fn new() -> Self {
    Self {
      game: Game::default(),
      state: Init(),
    }
  }

  #[must_use]
  pub fn connect(self) -> Result<Client<ClientConnected>, String> {
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:8754").unwrap();

    let mut packet_type: [u8; 1] = [0; 1];
    let _ = stream.peek(&mut packet_type);

    let opt_packet: Packet = if packet_type[0] == 2 {
      let mut buf: [u8; 2] = [0; 2];
      stream.read_exact(&mut buf).map_err(|err| err.to_string())?;
      Packet::from_bytes(&buf)
    } else {
      panic!()
    };
    let player: Player = if let Packet::AssignPlayer { player } = opt_packet {
      player
    } else {
      panic!()
    };
    Ok(Client::<ClientConnected> {
      game: self.game,
      state: ClientConnected {
        server: ServerHandler { stream },
        player,
      },
    })
  }
}

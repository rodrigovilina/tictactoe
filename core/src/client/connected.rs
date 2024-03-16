use {
  crate::{CellPosition, Client, ClientConnected, Packet, Player},
  std::{io::Write, thread::sleep, time::Duration},
};

impl Client<ClientConnected> {
  pub fn play(&mut self) {
    loop {
      self.tick()
    }
  }

  fn tick(&mut self) {
    let incoming_packet: Option<Packet> = self.get_packet();
    println!("incoming_packet: {:?}", incoming_packet);

    if let Some(packet) = incoming_packet {
      let outgoing_packet: Option<Packet> = self.update(packet);
      println!("outgoing_packet: {:?}", outgoing_packet);

      if let Some(o_packet) = outgoing_packet {
        self.send_packet(o_packet);
      }
    }
    println!("updating game state");
    let player = Player::X;
    let packet = Packet::AttemptPlay {
      player,
      position: CellPosition::Top,
    };
    self.send_packet(packet);
    sleep(Duration::from_millis(100));
  }

  fn update(&mut self, _packet: Packet) -> Option<Packet> {
    None
  }

  fn get_packet(&mut self) -> Option<Packet> {
    None
  }

  fn send_packet(&mut self, packet: Packet) {
    let bytes: usize = self.state.server.stream.write(&packet.to_bytes()).unwrap();
    println!("Wrote {} bytes", bytes);
  }
}

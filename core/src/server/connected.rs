use {
  crate::{AttemptPlay, Connected, Packet, Player, Server},
  std::{
    io::{Read, Write},
    thread::sleep,
    time::Duration,
  },
};

impl Server<Connected> {
  pub fn play(&mut self) {
    self.state.player_x.stream.set_nonblocking(true).unwrap();
    self.state.player_o.stream.set_nonblocking(true).unwrap();
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
      };
    };

    sleep(Duration::from_millis(100));
  }

  fn update(&mut self, packet: Packet) -> Option<Packet> {
    println!("updating game state");
    match packet {
      Packet::AttemptPlay { player, position } => {
        if player == self.game.current_turn {
          self.game.attempt_play(player, position);
          Some(Packet::Play {
            value: player.to_cell_value(),
            position,
          })
        } else {
          None
        }
      }
      Packet::Play { .. } => None,
    }
  }

  fn send_packet(&mut self, packet: Packet) {
    let bytes: usize = self
      .state
      .player_x
      .stream
      .write(&packet.to_bytes())
      .unwrap();
    println!("Wrote {} bytes to Player::X", bytes);
    let bytes: usize = self
      .state
      .player_o
      .stream
      .write(&packet.to_bytes())
      .unwrap();
    println!("Wrote {} bytes to Player::O", bytes);
  }

  fn get_packet(&mut self) -> Option<Packet> {
    let mut stream = match self.game.current_turn {
      Player::X => &self.state.player_x.stream,
      Player::O => &self.state.player_o.stream,
    };

    let mut packet_type: [u8; 1] = [0; 1];

    match stream.peek(&mut packet_type) {
      Ok(1) => {}
      Ok(0) => return None,
      Ok(_) => panic!(),
      Err(_) => return None,
    }

    println!("Peaked {} bytes as packet_type", 1);

    match packet_type[0] {
      0 => {
        let mut buf: [u8; 3] = [0; 3];
        stream.read_exact(&mut buf).ok()?;
        Some(Packet::from_bytes(buf.to_vec()))
      }
      _ => None,
    }
  }
}

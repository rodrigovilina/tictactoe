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
      self.tick();
    }
  }

  fn tick(&mut self) {
    let incoming_packet: Option<Packet> = self.get_packet();

    if let Some(packet) = incoming_packet {
      let outgoing_packet: Option<Packet> = self.update(packet);

      println!("outgoing_packet: {outgoing_packet:?}");

      if let Some(o_packet) = outgoing_packet {
        self.send_packet(o_packet);
      };
    };

    sleep(Duration::from_millis(1000));
  }

  fn update(&mut self, packet: Packet) -> Option<Packet> {
    println!("updating game state");
    match packet {
      Packet::AttemptPlay { player, position } => {
        if player == self.game.current_turn {
          let successful_play: bool = self.game.attempt_play(player, position);
          println!("{successful_play:?}");
          if successful_play {
            Some(Packet::Play {
              value: player.to_cell_value(),
              position,
            })
          } else {
            None
          }
        } else {
          None
        }
      }
      Packet::Play { .. } | Packet::AssignPlayer { .. } => None,
    }
  }

  fn send_packet(&mut self, packet: Packet) {
    let bytes: usize = self
      .state
      .player_x
      .stream
      .write(&packet.to_bytes())
      .unwrap();
    println!("Wrote {bytes} bytes to Player::X");
    let bytes: usize = self
      .state
      .player_o
      .stream
      .write(&packet.to_bytes())
      .unwrap();
    println!("Wrote {bytes} bytes to Player::O");
  }

  fn get_packet(&mut self) -> Option<Packet> {
    println!("Getting Packet from Clients");

    let mut stream = match self.game.current_turn {
      Player::X => &self.state.player_x.stream,
      Player::O => &self.state.player_o.stream,
    };

    let mut packet_type: [u8; 1] = [0; 1];

    match stream.peek(&mut packet_type) {
      Ok(1) => {
        let pt = packet_type[0];
        println!("Successfully read packet type: {pt:?}");
      }
      Ok(0) | Err(_) => return None,
      Ok(_) => panic!(),
    }

    match packet_type[0] {
      0 => {
        let mut buf: [u8; 3] = [0; 3];
        stream.read_exact(&mut buf).ok()?;
        let opt_packet = Some(Packet::from_bytes(&buf));
        println!("opt_packet: {opt_packet:?}");
        opt_packet
      }
      _ => None,
    }
  }
}

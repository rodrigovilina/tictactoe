use {
  crate::{CellPosition, Client, ClientConnected, Packet, Player},
  std::{io::{self, Read, Write}, net::TcpStream, thread::sleep, time::Duration},
};

impl Client<ClientConnected> {
  pub fn play(&mut self) {
    self.state.server.stream.set_nonblocking(true).unwrap();
    loop {
      self.tick();
    }
  }

  fn tick(&mut self) {
    let incoming_packet: Option<Packet> = self.get_packet();

    let outgoing_packet: Option<Packet> = self.update(incoming_packet);
    println!("outgoing_packet: {outgoing_packet:?}");

    self.send_packet(outgoing_packet);
    println!("updating game state");

    sleep(Duration::from_millis(1000));
  }

  fn update(&mut self, packet: Option<Packet>) -> Option<Packet> {
    match packet {
      Some(Packet::Play { value, position }) => {
        self.game.play(value, position);
        None
      }
      Some(_) | None => {
        let player = self.state.player;
        if player != self.game.current_turn {
          return None
        }
        let mut pos = String::new();
        self.game.draw();
        println!("Please input your play.");
        io::stdin()
          .read_line(&mut pos)
          .expect("Failed to read line");
        let pos: u8 = pos.trim().parse().map_or_else(|_| panic!(), |num| num);
        let position = CellPosition::from_byte(pos);

        let packet = Packet::AttemptPlay {
          player,
          position
        };
        Some(packet)
      }
    }
  }

  fn get_packet(&mut self) -> Option<Packet> {
    println!("Getting Packet from Server");
    let mut packet_type: [u8; 1] = [0; 1];
    let stream: &mut TcpStream = &mut self.state.server.stream;

    match stream.peek(&mut packet_type) {
      Ok(1) => {
        let pt = packet_type[0];
        println!("Successfully peaked packet type: {pt:?}");
      }
      Ok(0) | Err(_) => return None,
      Ok(_) => panic!(),
    }


    match packet_type[0] {
      1 => {
        let mut buf: [u8; 3] = [0; 3];
        stream.read_exact(&mut buf).ok()?;
        Some(Packet::from_bytes(&buf))
      }
      _ => None,
    }
  }

  fn send_packet(&mut self, packet: Option<Packet>) {
    let Some(packet) = packet else { return };

    let bytes: usize = self.state.server.stream.write(&packet.to_bytes()).unwrap();
    println!("Wrote {bytes} bytes");
  }
}

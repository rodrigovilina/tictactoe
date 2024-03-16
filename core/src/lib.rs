mod board;
mod cell_position;
mod cell_value;
mod client;
mod game;
mod packet;
mod player;
mod server;

use std::
  net::{TcpListener, TcpStream}
;

#[derive(Clone, Debug, Default, PartialEq)]
struct Board {
  pub top_left: CellValue,
  pub top: CellValue,
  pub top_right: CellValue,
  pub left: CellValue,
  pub center: CellValue,
  pub right: CellValue,
  pub bottom_left: CellValue,
  pub bottom: CellValue,
  pub bottom_right: CellValue,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CellValue {
  #[default]
  Empty,
  X,
  O,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Player {
  #[default]
  X,
  O,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellPosition {
  TopLeft,
  Top,
  TopRight,
  Left,
  Center,
  Right,
  BottomLeft,
  Bottom,
  BottomRight,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Game {
  board: Board,
  current_turn: Player,
}

pub struct Client<T: ClientState> {
  game: Game,
  state: T,
}

pub struct Server<T: ServerState> {
  game: Game,
  state: T,
}

pub struct ClientHandler {
  pub stream: TcpStream,
}

pub struct ServerHandler {
  pub stream: TcpStream,
}

mod private {
  pub trait PrivateServerState {}
  pub trait PrivateClientState {}
}

pub trait ServerState: private::PrivateServerState {}
pub trait ClientState: private::PrivateClientState {}

pub struct Init();
pub struct Listening {
  tcp_listener: TcpListener,
}
pub struct Connected {
  player_x: ClientHandler,
  player_o: ClientHandler,
}
pub struct ClientConnected {
  server: ServerHandler,
}

impl private::PrivateServerState for Init {}
impl private::PrivateClientState for Init {}
impl private::PrivateServerState for Listening {}
impl private::PrivateServerState for Connected {}
impl private::PrivateClientState for ClientConnected {}
impl ServerState for Init {}
impl ClientState for Init {}
impl ServerState for Listening {}
impl ServerState for Connected {}
impl ClientState for ClientConnected {}

pub trait AttemptPlay {
  fn attempt_play(&mut self, player: Player, position: CellPosition);
}

#[derive(Debug)]
enum Packet {
  AttemptPlay {
    player: Player,
    position: CellPosition,
  },
  Play {
    value: CellValue,
    position: CellPosition,
  },
}

//    //////////////////////////////////////////////////////////////////////////

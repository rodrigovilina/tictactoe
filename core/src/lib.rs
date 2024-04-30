#![warn(clippy::complexity)]
#![warn(clippy::expect_used)]
#![warn(clippy::nursery)]
#![warn(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::unwrap_used)]

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

#[derive(Clone, Copy, Debug)]
enum Packet {
  AttemptPlay {
    player: Player,
    position: CellPosition,
  },
  Play {
    value: CellValue,
    position: CellPosition,
  },
  AssignPlayer {
    player: Player
  }
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

pub trait AttemptPlay {
  fn attempt_play(&mut self, player: Player, position: CellPosition) -> bool;
}

// Server and Client States

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
  player: Player,
}

mod private {
  pub trait ServerState {}
  pub trait ClientState {}
}

pub trait ServerState: private::ServerState {}
pub trait ClientState: private::ClientState {}

impl private::ServerState for Init {}
impl private::ClientState for Init {}
impl private::ServerState for Listening {}
impl private::ServerState for Connected {}
impl private::ClientState for ClientConnected {}

impl ServerState for Init {}
impl ClientState for Init {}
impl ServerState for Listening {}
impl ServerState for Connected {}
impl ClientState for ClientConnected {}



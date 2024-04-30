use core::Client;

fn main() {
  Client::new().connect().unwrap().play();
}

use core::{Init, Server};

fn main() {
  Server::<Init>::new().listen().connect().play();
}

use std::net::TcpStream;

use crate::network::{packet_io::write_packet, packets::new_client_packet::NewClientPacket};

use super::game_state::GameState;

#[allow(dead_code)]
pub fn init() {
    let mut stream = Box::new(TcpStream::connect("127.0.0.1:14727").unwrap());
    let mut game_state = GameState::default();

    let join_packet = NewClientPacket::default();
    write_packet(&mut stream, Box::new(join_packet));
}
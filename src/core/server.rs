use std::net::{TcpListener, TcpStream};

use crate::network::{packet_io::read_packet, packets::Packets};

use super::game_state::GameState;

fn client_handler(mut stream: Box<TcpStream>, game_state: &mut GameState) -> bool {
    let packets = Packets::get_packets();
    let connection_result = read_packet(&mut stream, &packets);
    let connection = match connection_result {
        Ok(val) => val,
        Err(e) => {
            println!("Connection Failed: {e}");
            return false;
        }
    };

    connection.operate(game_state, stream);
    return true;
}

#[allow(dead_code)]
pub fn init() {
    let listener = TcpListener::bind("0.0.0.0:14727").unwrap();
    let mut game_state = GameState::default();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        if !client_handler(Box::new(stream), &mut game_state) {
            println!("Failed to connect client.");
        }
    }
}
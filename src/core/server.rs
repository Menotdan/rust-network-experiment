use std::net::{TcpListener, TcpStream};

use crate::network::{packet_io::read_packet, packets::Packets};

use super::game_state::GameState;

fn client_handler(mut stream: Box<TcpStream>, game_state: &mut GameState) -> bool {
    let packets = Packets::get_packets();
    let connection_result = read_packet(&mut stream, &packets);
    let connection = match connection_result {
        Ok(val) => match val {
            Ok(real_val) => real_val,
            Err(_) => {
                println!("No connection packet to read!");
                return false;
            }
        },
        Err(e) => {
            println!("Connection Failed: {e}");
            return false;
        }
    };

    let _ = stream.set_nonblocking(true);
    connection.operate(game_state, stream);
    return true;
}

fn tick(game_state: &mut GameState) {

}

#[allow(dead_code)]
pub fn init() {
    let listener = TcpListener::bind("0.0.0.0:14727").unwrap();
    let mut game_state = GameState::default();

    loop {
        // Look for new clients
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("{}", stream.peer_addr().unwrap());
            if !client_handler(Box::new(stream), &mut game_state) {
                println!("Failed to connect client.");
            }
        }

        tick(&mut game_state);
    }
}
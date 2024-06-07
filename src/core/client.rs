use std::{net::TcpStream, thread, time::Duration};

use crate::network::{packet::Packet, packet_io::{read_packet, write_packet}, packets::{new_client_packet::NewClientPacket, Packets}};

use super::game_state::GameState;

#[allow(dead_code)]
pub fn init() {
    let mut stream = Box::new(TcpStream::connect("127.0.0.1:14727").unwrap());
    let _ = stream.set_nonblocking(true);
    let mut game_state = GameState::default();
    let packets = Packets::get_packets();

    // Join process, get new client ID.
    let mut join_packet = NewClientPacket::default();
    join_packet.meow = 727;
    let join_packet_box: Box<dyn Packet> = Box::new(join_packet);
    let _ = write_packet(&mut stream, &join_packet_box);

    thread::sleep(Duration::from_millis(500));
    let client_id_packet = read_packet(&mut stream, &packets);
    let client_id = match client_id_packet {
        Ok(val) => match val {
            Ok(val) => {
                let _ = val.operate(&mut game_state, stream);
                val.get_client_id()
            },
            Err(_) => {
                println!("err!");
                return;
            }
        },
        Err(err) => {
            println!("err: {}", err);
            return;
        }
    };

    println!("client id: {}", client_id);

    loop {
        // TODO: game
    }
}
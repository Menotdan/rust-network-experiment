use std::{net::TcpStream, thread, time::Duration};

use crate::network::{packet::{Packet, PacketType}, packet_io::{read_packet, write_packet}, packets::{new_client_packet::NewClientPacket, Packets}};

use super::game_state::GameState;

fn add_new_client(new_client: &Box<NewClientPacket>) {

}

fn recieve_client_id(id_packet: &PacketType) -> Result<u32, String> {
    match id_packet {
        PacketType::GenericPacket(_) => return Err(String::from("Did not recieve client id packet.")),
        PacketType::NewClientPacket(packet) => {
            return Ok(packet.get_client_id());
        }
    }
}

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
            Ok(packet) => {
                match recieve_client_id(&packet) {
                    Ok(id) => id,
                    Err(msg) => {
                        println!("Connection failed: {msg}");
                        return;
                    }
                }
            },
            Err(_) => return
        },
        Err(_) => return
    };

    println!("client id: {}", client_id);

    loop {
        // TODO: game
    }
}
use std::{net::TcpStream, thread, time::Duration};

use crate::{core::game_state::Client, network::{packet::{Packet, PacketType}, packet_io::{read_packet, write_packet}, packets::{new_client_packet::NewClientPacket, Packets}}, types::pos::Pos};

use super::game_state::GameState;

fn add_new_client(game_state: &mut GameState, new_client_packet: &Box<NewClientPacket>) {
    let new_client = Client { id: new_client_packet.get_client_id(), position: Pos::default(), stream: Option::None, disconnected: false };

    game_state.clients.insert(new_client_packet.get_client_id(), new_client);
    
    println!("New client with ID {}, meow: {}", new_client_packet.get_client_id(), new_client_packet.meow);
}

fn recieve_client_id(id_packet: &PacketType) -> Result<u32, String> {
    match id_packet {
        PacketType::GenericPacket(_) => return Err(String::from("Did not recieve client id packet.")),
        PacketType::NewClientPacket(packet) => {
            return Ok(packet.get_client_id());
        }
    }
}

fn tick(game_state: &mut GameState, stream: &mut Box<TcpStream>) -> bool {
    let packets = Packets::get_packets();
    loop {
        let read_result = read_packet(stream, &packets);
        let packet = match read_result {
            Ok(val) => match val {
                Ok(real_val) => real_val,
                Err(_) => {
                    break;
                }
            },
            Err(_) => {
                return false;
            }
        };

        if let PacketType::GenericPacket(packet) = packet {
            let target_client = game_state.clients.get_mut(&packet.get_client_id()).expect("wtff");
            let _ = packet.operate(target_client, &mut game_state.data);
        } else if let PacketType::NewClientPacket(packet) = packet {
            add_new_client(game_state, &packet);
        }
    }

    return true;
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

    thread::sleep(Duration::from_millis(50));
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
        thread::sleep(Duration::from_millis(50));
        tick(&mut game_state, &mut stream);
    }
}
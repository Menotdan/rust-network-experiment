use std::net::{TcpListener, TcpStream};

use crate::{core::game_state::Client, network::{packet::{Packet, PacketType}, packet_io::{read_packet, write_packet}, packets::{new_client_packet::NewClientPacket, Packets}}, types::pos::Pos};

use super::game_state::GameState;

fn client_connection_handler(mut stream: Box<TcpStream>, game_state: &mut GameState) -> bool {
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

    let broadcast = match connection {
        PacketType::NewClientPacket(packet) => {
            let mut i = packet.get_client_id();
            loop {
                if !game_state.clients.contains_key(&i) {
                    break;
                }
                i += 1;
            }

            let new_client = Client { id: i, position: Pos::default(), stream: stream, disconnected: false };
            let _ = new_client.stream.set_nonblocking(true);
            game_state.clients.insert(i, new_client);
            println!("New client with ID {}, meow: {}", i, packet.meow);

            let mut output_packet = NewClientPacket::default();
            output_packet.meow = packet.meow;
            output_packet.set_client_id(i);

            let output_box: Box<dyn Packet> = Box::new(output_packet);
            output_box
        },
        _ => {
            println!("Client sent invalid connection packet!");
            return false;
        }
    };

    for client in game_state.clients.values_mut() {
        let _ = write_packet(&mut client.stream, &broadcast);
    }

    return true;
}

fn tick(game_state: &mut GameState) {
    let packets = Packets::get_packets();
    let mut broadcast_packets: Vec<Box<dyn Packet>> = Vec::new();
    
    // Handle packets and accumulate broadcast packets
    for client in game_state.clients.values_mut() {
        loop {
            let read_result = read_packet(&mut client.stream, &packets);
            let packet = match read_result {
                Ok(val) => match val {
                    Ok(real_val) => real_val,
                    Err(_) => {
                        break;
                    }
                },
                Err(e) => {
                    client.disconnected = true;
                    break;
                }
            };

            if let PacketType::GenericPacket(packet) = packet {
                let broadcast = packet.operate(client, &mut game_state.data);
                match broadcast {
                    Ok(val) => {
                        if let PacketType::GenericPacket(broadcast_packet) = val {
                            broadcast_packets.push(broadcast_packet);
                        }
                    },
                    _ => { return; }
                }
            }
        }
    }

    // Broadcast packets
    for packet in broadcast_packets {
        for client in game_state.clients.values_mut() {
            if client.disconnected {
                continue;
            }

            let _ = write_packet(&mut client.stream, &packet);
        }
    }

    game_state.clients.retain(|_, client| !client.disconnected);
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
            if !client_connection_handler(Box::new(stream), &mut game_state) {
                println!("Failed to connect client.");
            }
        }

        tick(&mut game_state);
    }
}
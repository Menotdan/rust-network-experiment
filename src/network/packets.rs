use move_packet::MovePacket;
use new_client_packet::NewClientPacket;

use super::packet::{Packet, PacketType};

pub mod move_packet;
pub mod new_client_packet;

pub struct Packets {
    packets: Vec<PacketType>,
}

impl Packets {
    pub fn get_new_packet_from_id(&self, id: u32) -> Result<PacketType, String> {
        for p in &self.packets {
            match p {
                PacketType::NewClientPacket(packet) => {
                    if packet.get_id() == id {
                        return Ok(packet.get_new());
                    }
                },
                PacketType::GenericPacket(packet) => {
                    if packet.get_id() == id {
                        return Ok(packet.get_new());
                    }
                }
            }
        }

        return Err(String::from("No packet found!"));
    }

    pub fn get_packets() -> Packets {
        let mut new_packets = Packets { packets: Vec::new() };
        new_packets.packets.push(PacketType::NewClientPacket(Box::new(NewClientPacket::default())));
        new_packets.packets.push(PacketType::GenericPacket(Box::new(MovePacket::default())));

        return new_packets;
    }
}
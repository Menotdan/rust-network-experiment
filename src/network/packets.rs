use move_packet::MovePacket;
use new_client_packet::NewClientPacket;

use super::packet::Packet;

pub mod move_packet;
pub mod new_client_packet;

pub struct Packets {
    packets: Vec<Box<dyn Packet>>,
}

impl Packets {
    pub fn get_new_packet_from_id(&self, id: u32) -> Result<Box<dyn Packet>, String> {
        for p in &self.packets {
            if p.get_id() == id {
                return Ok(p.get_new());
            }
        }

        return Err(String::from("No packet found!"));
    }

    pub fn get_packets() -> Packets {
        let mut new_packets = Packets { packets: Vec::<Box<dyn Packet>>::new() };
        new_packets.packets.push(Box::new(NewClientPacket::default()));
        new_packets.packets.push(Box::new(MovePacket::default()));

        return new_packets;
    }
}